use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Binding port
    #[arg(short, long)]
    pub port: u16,

    /// Listen addr
    #[arg(short, long)]
    pub addr: String
}
