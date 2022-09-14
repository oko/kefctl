use clap::Parser;
use libkef::{Command, Source, Volume};
use std::{
    io::Write,
    net::{IpAddr, SocketAddr, TcpStream},
};

/// Control program for KEF LSX and similar speakers
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// IP address of the KEF speakers to control
    #[clap(short = 'i', long = "ip", value_parser)]
    ip: IpAddr,
    /// TCP port of the KEF speakers to control
    #[clap(short = 'p', long = "port", value_parser, default_value = "50001")]
    port: u16,
    /// Source input
    #[clap(short = 's', long = "source", value_parser)]
    source: Option<Source>,
    /// Volume 0-100
    #[clap(short = 'v', long = "volume", value_parser)]
    volume: Option<Volume>,
    /// Turn off the speakers (only command sent if present)
    #[clap(short = 'x', long = "off", value_parser)]
    off: bool,
}

fn main() {
    let args = Args::parse();
    eprintln!("args: {:?}", args);

    // TODO: make this fail more gracefully
    let mut conn = TcpStream::connect(SocketAddr::new(args.ip, args.port)).unwrap();

    // If we're turning off, just do that and ignore everything else
    if args.off {
        conn.write(Command::TurnOff.to_bytes().as_slice()).unwrap();
        return;
    }

    // Set source if specified
    match args.source {
        Some(source) => {
            conn.write(source.to_bytes().as_slice()).unwrap();
        }
        None => {}
    }

    // Set volume if specified
    match args.volume {
        Some(vol) => {
            conn.write(vol.to_bytes().as_slice()).unwrap();
        }
        None => {}
    }
}
