use cli::Cli;

mod blockchain;
mod cli;

fn main() {
    let mut cli = Cli::new().expect("Error to instance cli");
    let _ = cli.run();
}
