use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Firewall {
    pub firewall: FirewallRules
}

#[derive(Serialize, Deserialize)]
pub struct FirewallRules {
    inbound_rules: Vec<Rule>
}

#[derive(Serialize, Deserialize)]
pub struct Rule {
    protocol: String,
    ports: String,
    sources: Sources,
}

#[derive(Serialize, Deserialize)]
pub struct Sources {
    addresses: Vec<String>
}

impl FirewallRules {
    pub fn new(address: String) -> FirewallRules {
        return FirewallRules {
            inbound_rules: vec![
                Rule {
                    protocol: String::from("tcp"),
                    ports: String::from("22"),
                    sources: Sources {
                        addresses: vec![address]
                    },
                }
            ]
        };
    }

    pub fn from_addresses(addresses: Vec<String>) -> FirewallRules {
        return FirewallRules {
            inbound_rules: vec![
                Rule {
                    protocol: String::from("tcp"),
                    ports: String::from("22"),
                    sources: Sources {
                        addresses
                    },
                }
            ]
        };
    }
}

impl FirewallRules {
    pub fn list_of_ssh_addresses(&self) -> Vec<String> {
        return self.inbound_rules
            .iter()
            .filter(|rule| rule.ports == "22")
            .map(|rule| rule.sources.addresses.clone())
            .collect::<Vec<Vec<String>>>()
            .iter()
            .flatten()
            .cloned()
            .collect();
    }
}