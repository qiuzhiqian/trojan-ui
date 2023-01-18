pub async fn start(address: String,port: u32) -> Result<()> {
    // Extract the inbound client address
    let address = (inbound_config.address.clone(), inbound_config.port)
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    // Start the TCP server listener socket
    let listener = TcpListener::bind(address).await?;

    // Create TCP server acceptor and handler
    let (acceptor, handler) = (
        TcpAcceptor::init(&inbound_config),
        TcpHandler::init(&outbound_config),
    );

    // Enter server listener socket accept loop
    loop {
        info!("Ready to accept new socket connection");

        let (socket, addr) = listener.accept().await?;

        info!("Received new connection from {}", addr);

        let (acceptor, handler) = (acceptor, handler);

        tokio::spawn(async move {
            let (request, inbound_stream) = match acceptor.accept(socket).await {
                Ok(stream) => stream,
                Err(e) => {
                    warn!("Failed to accept inbound connection from {}: {}", addr, e);
                    return;
                }
            };

            match handler.dispatch(inbound_stream, request).await {
                Ok(_) => {
                    info!("Connection from {} has finished", addr);
                }
                Err(e) => {
                    warn!("Failed to handle the inbound stream: {}", e);
                }
            }
        });
    }
}

/// Handle inbound TCP stream with TCP outbound proxy strategy. This function is used when the program serves as
/// the client end of proxy chain, such that it read the plaintext data from the inbound stream and will encrypt
/// the it with the selected proxy and forward the proxy request to remote server.
async fn handle_tcp_stream<T: AsyncRead + AsyncWrite + Unpin + Send>(
    &self,
    request: InboundRequest,
    inbound_stream: StandardTcpStream<T>,
) -> io::Result<()> {
    // Establish the initial connection with remote server
    let connection = match self.destination {
        Some(dest) => TcpStream::connect(dest).await?,
        None => {
            return Err(Error::new(
                ErrorKind::NotConnected,
                "missing address of the remote server",
            ))
        }
    };

    // Escalate the connection to TLS connection if tls config is present
    let mut outbound_stream = match &self.tls {
        Some((client_config, domain)) => {
            let connector = TlsConnector::from(client_config.clone());
            StandardTcpStream::RustlsClient(
                connector.connect(domain.clone(), connection).await?,
            )
        }
        None => StandardTcpStream::Plain(connection),
    };

    // Handshake to form the proxy stream
    match self.protocol {
        SupportedProtocols::TROJAN => {
            // Check Trojan secret match
            if self.secret.len() != HEX_SIZE {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Hex in trojan protocol is not {} bytes", HEX_SIZE),
                ));
            }

            // Start handshake to establish proxy stream
            handshake(&mut outbound_stream, &request, &self.secret).await?;

            match request.transport_protocol {
                TransportProtocol::TCP => {
                    let (mut client_reader, mut client_writer) =
                        tokio::io::split(inbound_stream);
                    let (mut server_reader, mut server_writer) =
                        tokio::io::split(outbound_stream);

                    // Obtain reader and writer for inbound and outbound streams
                    tokio::select!(
                        _ = tokio::io::copy(&mut client_reader, &mut server_writer) => (),
                        _ = tokio::io::copy(&mut server_reader, &mut client_writer) => ()
                    );
                }
                TransportProtocol::UDP => {
                    let (client_reader, client_writer) = tokio::io::split(inbound_stream);
                    let (server_reader, server_writer) = tokio::io::split(outbound_stream);

                    tokio::select!(
                        _ = trojan::packet::copy_client_reader_to_udp_server_writer(client_reader, BufWriter::new(server_writer), request) => (),
                        _ = trojan::packet::copy_udp_server_reader_to_client_writer(BufReader::new(server_reader), client_writer) => (),
                    );
                }
            }
        }
        SupportedProtocols::SOCKS => {
            return Err(Error::new(ErrorKind::Unsupported, "Unsupported protocol"))
        }
        SupportedProtocols::DIRECT => {
            return Err(Error::new(ErrorKind::Unsupported, "Unsupported protocol"));
        }
    };

    info!("Connection finished");
    Ok(())
}
