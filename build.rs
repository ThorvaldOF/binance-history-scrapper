use {
    std::env,
    winres::WindowsResource,
};

fn main() {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            .set_icon("icon.ico")
            .compile().expect("TODO: panic message");
    }
}
