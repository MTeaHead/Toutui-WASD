use clap::Command;

pub fn clap() {
    let _matches = Command::new("toutui")
        .version(env!("CARGO_PKG_VERSION")) 
        .get_matches();
}
