use std::collections::BTreeSet;
use std::time::Duration;
use krakenrs::{KrakenRestConfig, KrakenRestAPI, MarketOrder, BsType, KrakenCredentials};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    key: String,
    passphrase: String,
    pair: String,
    volume: String,
    dry_run: bool
}

fn main() {
    env_logger::init();

    let file = std::fs::File::open("config.yml").expect("Could not open file.");
    let config: Config = serde_yaml::from_reader(file).expect("Could not read values.");

    if config.dry_run {
        warn!("⚠ you are running in dry-run mode. orders are not executed! ⚠");
    }

    let kc_config = KrakenRestConfig {
        timeout: Duration::from_secs(60),
        creds: KrakenCredentials {
            key: config.key,
            secret: config.passphrase,
        },
    };

    let api = KrakenRestAPI::try_from(kc_config).expect("could not create kraken api");

    let r = api.add_market_order(MarketOrder {
        bs_type: BsType::Buy,
        pair: config.pair,
        volume: config.volume,
        oflags: BTreeSet::new(),
    }, None, config.dry_run);

    if let Ok(response) = r {
        info!("💰 {}", response.descr.order);
    } else {
        error!("❌ {}", r.unwrap_err());
    }

    println!("press any key to exit...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
