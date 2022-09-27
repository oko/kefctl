use clap::Parser;
use log::{error, info};
use std::time::Duration;

#[derive(Parser, Debug)]
#[clap(name = "kefdisc")]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 't', long = "timeout")]
    timeout: u64,
    #[clap(short = 's', long = "serial")]
    serial: Option<String>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();
    match libkef::discovery::discover(Duration::from_secs(args.timeout)).await {
        Some(speakers) => match args.serial {
            Some(serial) => {
                for (u, sn) in speakers.iter() {
                    if &serial == sn {
                        println!("{}", u.host().unwrap());
                    }
                }
            }
            None => {
                for (u, sn) in speakers.iter() {
                    println!("{}\t{}", u.host().unwrap(), sn);
                }
                info!(
                    "discovered {} speakers in {} seconds",
                    speakers.keys().len(),
                    args.timeout
                );
            }
        },
        None => {
            error!("no speakers discovered in {} seconds", args.timeout)
        }
    }
}
