use clap::Parser;
use libkef::{Command, CommandResult, Inverse, Power, Source, Standby, Volume};
use log::{debug, error};
use std::net::{IpAddr, SocketAddr};

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
    /// Standby duration
    #[clap(short = 'S', long = "standby", value_parser)]
    standby: Option<Standby>,
    /// Volume 0-100
    #[clap(short = 'v', long = "volume", value_parser)]
    volume: Option<Volume>,
    /// Use the left speaker as the primary
    #[clap(short = 'P', long = "primary", value_parser)]
    primary: Option<Inverse>,
    /// Use the left speaker as the primary
    #[clap(short = 'o', long = "power", value_parser)]
    power: Option<Power>,
    /// Get speaker source settings
    #[clap(short = 'g', long = "get", value_parser)]
    get: bool,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();
    let sa = SocketAddr::new(args.ip, args.port);

    let (cur_power, cur_primary, cur_standby, cur_source) = match Command::GetSource.execute(&sa) {
        Ok(res) => match res {
            CommandResult::GotSource(pw, pr, sb, sr) => {
                debug!("got source response");
                (pw, pr, sb, sr)
            }
            _ => {
                error!("got invalid response type from GetSource command?");
                return;
            }
        },
        Err(_) => {
            println!("err");
            return;
        }
    };

    if args.get {
        println!("power: {:?}", cur_power);
        println!("primary: {:?}", cur_primary);
        println!("standby: {:?}", cur_standby);
        println!("source: {:?}", cur_source);
        return;
    }

    match Command::SetSource(
        match args.power {
            Some(pw) => pw,
            None => cur_power,
        },
        match args.primary {
            Some(pr) => pr,
            None => cur_primary,
        },
        match args.standby {
            Some(sb) => sb,
            None => cur_standby,
        },
        match args.source {
            Some(sr) => sr,
            None => cur_source,
        },
    )
    .execute(&sa)
    {
        Ok(_) => eprintln!("successfully set sources"),
        Err(_) => eprintln!("error setting sources"),
    };

    // Set volume if specified
    match args.volume {
        Some(vol) => match Command::SetVolume(vol).execute(&sa) {
            Ok(_) => eprintln!("successfully set volume"),
            Err(_) => eprintln!("error setting volume"),
        },
        None => {}
    }
}
