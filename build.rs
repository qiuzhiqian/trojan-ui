fn main() {
    slint_build::compile_with_config(
        "ui/appwindow.slint",
        slint_build::CompilerConfiguration::new(),
    ).unwrap();
}