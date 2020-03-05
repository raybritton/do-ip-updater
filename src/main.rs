mod models;
mod updater;

use clap::{App, Arg};
use simplelog::{ConfigBuilder, SimpleLogger};
use log::LevelFilter;
use crate::updater::Updater;
use std::time::Duration;

pub type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let matches = App::new("DigitalOcean IP Updater")
        .version("1.0")
        .author("Ray Britton <raybritton@gmail.com>")
        .about("Regularly checks that the internet IP address of this program is on a DigitalOcean firewall for the SSH port, and sets it if not")
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


