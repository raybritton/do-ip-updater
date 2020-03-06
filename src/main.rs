mod models;
mod updater;

use clap::{App, Arg, crate_version, crate_description, crate_authors, crate_name};
use simplelog::{ConfigBuilder, SimpleLogger};
use log::LevelFilter;
use crate::updater::Updater;
use std::time::Duration;

pub type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("firewall_id")
            .long("id")
            .short("i")
            .help("DigitalOcean Firewall ID")
            .required(true)
            .multiple(false)
            .value_name("ID")
            .takes_value(true)
            .number_of_values(1))
        .arg(Arg::with_name("do_token")
            .long("token")
            .short("t")
            .help("DigitalOcean Bearer Token")
            .required(true)
            .multiple(false)
            .takes_value(true)
            .value_name("TOKEN")
            .number_of_values(1))
        .arg(Arg::with_name("freq")
            .long("frequency")
            .short("f")
            .help("How often (in minutes) to check the IP address is set")
            .required(false)
            .multiple(false)
            .takes_value(true)
            .number_of_values(1)
            .value_name("MINUTES")
            .default_value("30")
            .validator(|value| value.parse::<u64>().map(|_| ()).map_err(|_| String::from("Must be positive whole number"))))
        .arg(Arg::with_name("port")
            .long("port")
            .short("p")
            .help("Port in the firewall to be updated")
            .required(false)
            .multiple(false)
            .takes_value(true)
            .number_of_values(1)
            .value_name("PORT")
            .default_value("22")
            .validator(|value| value.parse::<u64>().map(|_| ()).map_err(|_| String::from("Must be positive whole number"))))
        .arg(Arg::with_name("once")
            .long("once")
            .short("o")
            .help("Run once then exit")
            .required(false)
            .multiple(false)
            .takes_value(false))
        .arg(Arg::with_name("verbose")
            .takes_value(false)
            .short("v")
            .long("verbose")
            .help("Set verbosity of program (between 0 and 3)")
            .required(false)
            .multiple(true))
        .get_matches();

    let token = matches.value_of("do_token").expect("[CLAP ERROR] No DigitalOcean token").to_string();
    let id = matches.value_of("firewall_id").expect("[CLAP ERROR] No DigitalOcean id").to_string();
    let freq: u64 = matches.value_of("freq").expect("[CLAP ERROR] No frequency").parse().unwrap();
    let port = matches.value_of("port").expect("[CLAP ERROR] No port").parse().unwrap();
    let once = matches.is_present("once");
    let verbosity = matches.occurrences_of("verbose");

    let log_level = int_to_log_level(verbosity);

    let config = ConfigBuilder::new()
        .set_thread_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Off)
        .set_location_level(LevelFilter::Trace)
        .add_filter_ignore_str("hyper")
        .add_filter_ignore_str("want")
        .add_filter_ignore_str("mio")
        .add_filter_ignore_str("reqwest")
        .build();

    SimpleLogger::init(log_level, config)?;

    let updater = Updater::new(
        id,
        token,
        Duration::from_secs(freq * 60),
        port,
        once
    );

    updater.run()?;

    Ok(())
}

fn int_to_log_level(count: u64) -> log::LevelFilter {
    return match count.min(3) {
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        3 => log::LevelFilter::Trace,
        _ => log::LevelFilter::Error
    };
}


