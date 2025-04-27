use clap::Command;

pub fn clap() {
    let matches = Command::new("toutui")
        .version(env!("CARGO_PKG_VERSION")) 
        .get_matches();

    if matches.args_present() {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return;
    }
}
