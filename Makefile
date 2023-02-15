all: build

build: trojan_ui

trojan_ui:
	cargo build -r

install:
	install -Dm755 target/release/trojan_ui ${DESTDIR}/usr/bin/trojan_ui
	install -Dv -m0644 config/trojan_ui.desktop ${DESTDIR}/usr/share/applications/trojan_ui.desktop
	install -Dv -m0644 config/trojan_ui.svg ${DESTDIR}/usr/share/icons/hicolor/scalable/apps/trojan_ui.svg

clean:
	rm -rf target/release