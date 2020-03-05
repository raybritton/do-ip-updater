use std::time::Duration;
use crate::Error;
use crate::models::{FirewallRules, Firewall};
use log::{error, trace, info};
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;

const GET_IP_ENDPOINT: &'static str = "https://status-api.dokku-ray.app/ip?format=text";
const HEADER_AUTH: &str = "Authorization";

pub struct Updater {
    get_ip: String,
    firewall_endpoint: String,
    token: String,
    freq: Duration,
    port: usize,
    once: bool
}

impl Updater {
    pub fn new(id: String, token: String, freq: Duration, port: usize, once: bool) -> Updater {
        return Updater {
            get_ip: String::from(GET_IP_ENDPOINT),
            firewall_endpoint: format!("https://api.digitalocean.com/v2/firewalls/{}", id),
            token,
            freq,
            port,
            once
        };
    }
}

impl Updater {
    pub fn run(&self) -> Result<(), Error> {
        let mut last_ip: Option<String> = None;
        let timeout = Duration::from_secs(180);
        let mut headers = HeaderMap::new();
        headers.insert(HEADER_AUTH, format!("Bearer {}", self.token).parse().unwrap());
        let client = reqwest::blocking::ClientBuilder::new()
            .default_headers(headers)
            .connect_timeout(timeout)
            .timeout(timeout)
            .build()?;

        info!("Monitoring");

        loop {
            let ip_resp = self.get_internet_ip(&client);

            if ip_resp.is_err() {
                error!("{}", ip_resp.unwrap_err().description());
                continue;
            }

            let ip = ip_resp.unwrap();

            if last_ip.is_none() || last_ip.as_ref().unwrap() != &ip {
                last_ip = Some(ip.clone());

                info!("IP has changed to {}", ip);

                let ssh_addresses_result = self.get_firewall_ssh_ips(&client);

                if let Err(err) = ssh_addresses_result {
                    error!("{}", err.description());
                    continue;
                }

                let delete_result = self.delete_rules(&client, ssh_addresses_result.unwrap());

                if let Err(err) = delete_result {
                    error!("{}", err.description());
                    continue;
                }

                match self.post_rules(&client, ip) {
                    Ok(text) => {
                        trace!("{}", text);
                    }
                    Err(err) => {
                        error!("{}", err.description());
                    }
                }
            } else {
                trace!("IP hasn't changed");
            }

            if self.once {
                return Ok(());
            }
            trace!("Checking in {} minutes", self.freq.as_secs() / 60);
            std::thread::sleep(self.freq)
        }
    }

    fn get_internet_ip(&self, client: &Client) -> Result<String, Error> {
        let resp = client.get(&self.get_ip).send()?.text();
        return if resp.is_err() {
            Err(Error::from(format!("Error during getting IP address: {}", resp.unwrap_err())))
        } else {
            Ok(resp.unwrap())
        };
    }

    fn get_firewall_ssh_ips(&self, client: &Client) -> Result<Vec<String>, Error> {
        let resp = client.get(&self.firewall_endpoint)
            .send();

        return if resp.is_err() {
            Err(Error::from(format!("Error getting rules: {}", resp.unwrap_err())))
        } else {
            match resp.unwrap().json::<Firewall>() {
                Ok(rules) => Ok(rules.firewall.list_of_addresses(self.port)),
                Err(err) => Err(Error::from(format!("Error parsing rules: {}", err)))
            }
        };
    }

    fn delete_rules(&self, client: &Client, addresses: Vec<String>) -> Result<(), Error> {
        trace!("Removing {}", addresses.join(", "));
        let rules = FirewallRules::from_addresses(addresses, self.port);
        let resp = client.delete(&format!("{}/rules", &self.firewall_endpoint))
            .json(&rules)
            .send();

        return if resp.is_err() {
            Err(Error::from(format!("Error deleting rules: {}", resp.unwrap_err())))
        } else {
            Ok(())
        };
    }

    fn post_rules(&self, client: &Client, ip: String) -> Result<String, Error> {
        let rules = FirewallRules::new(ip, self.port);

        let resp = client.post(&format!("{}/rules", &self.firewall_endpoint))
            .json(&rules)
            .send();

        return if resp.is_err() {
            Err(Error::from(format!("Error during setting rules: {}", resp.unwrap_err())))
        } else if !resp.as_ref().unwrap().status().is_success() {
            Err(Error::from(format!("Error during setting rules: {}", resp.unwrap().status())))
        } else {
            Ok(String::from("IP updated"))
        };
    }
}