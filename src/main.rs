use std::{io::Write, net::TcpStream};

use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    pub filename: String,
    pub line: String,
    pub column: String,
}

type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> DynResult<()> {
    let cli = Cli::parse();

    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:14905")?;

    // Send a message and don't wait for response
    let message = format!("{}:{}:{}\n", cli.filename, cli.line, cli.column);
    stream.write_all(message.as_bytes())?;

    Ok(())
}
