use clap::App;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("cargo_pkg_version"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();
}