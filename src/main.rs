use bytes::buf::Reader;
use bytes::Buf;
use clap::Parser;
use libkef::{Command, Source, Volume};
use ssdp::{
    header::{HeaderMut, HeaderRef, Location, Man, Server, MX, ST},
    message::{Multicast, SearchRequest},
};
use std::{
    collections::HashMap,
    hash::Hash,
    io::{BufReader, Write},
    net::{IpAddr, SocketAddr, TcpStream},
    str::FromStr,
};
use url::Url;
use xmltree::Element;

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
    #[clap(short = 'D', long = "discover", value_parser)]
    discover: bool,
}

fn discover() {
    let mut request = SearchRequest::new();
    request.set(Man);
    request.set(MX(1));
    request.set(ST::All);

    let mut found: HashMap<String, bool> = HashMap::new();
    let mut speakers: HashMap<String, String> = HashMap::new();

    for (msg, _src) in request.multicast().unwrap() {
        match msg.get::<Location>() {
            Some(x) => match Url::parse(x.as_str()) {
                Ok(u) => {
                    let target = u.host_str().unwrap().to_owned();

                    // no need to hammer already-discovered targets
                    if found.contains_key(&target) {
                        continue;
                    }

                    let resp = reqwest::blocking::get(u.as_str()).unwrap();
                    let respBytes = resp.bytes().unwrap();
                    let mut root = Element::parse(respBytes.clone().reader()).unwrap();
                    match root.get_child("device") {
                        Some(device) => match device.get_child("manufacturer") {
                            Some(mfg) => match mfg.get_text() {
                                Some(text) => {
                                    found.insert(target.clone(), text == "KEF");
                                    if text == "KEF" {
                                        speakers.insert(
                                            target.clone(),
                                            match device.get_child("serialNumber") {
                                                Some(serial_elem) => {
                                                    serial_elem.get_text().unwrap().to_string()
                                                }
                                                None => "missing".to_owned(),
                                            },
                                        );
                                    }
                                }
                                None => (),
                            },
                            None => (),
                        },
                        None => (),
                    }
                }
                Err(e) => {
                    eprintln!("error: {:?}", e)
                }
            },
            None => (),
        }
    }

    for (k, v) in speakers.iter() {
        println!("{}\t{}", k, v);
    }
}

fn main() {
    let args = Args::parse();
    eprintln!("args: {:?}", args);

    if args.discover {
        eprintln!("running discovery");
        discover();
        return;
    }

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
