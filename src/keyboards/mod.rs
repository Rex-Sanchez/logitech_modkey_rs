use std::str::FromStr;

pub mod g15;
pub mod g19;

#[derive(Debug, Clone)]
pub enum KbVersion {
    G15,
    G19,
}

pub struct DevConfig {
    pub dev_name: &'static str,
    pub interface: u8,
    pub configuration: u8,
    pub endpoint: u8,
    pub friendly_name: &'static str,
}

impl KbVersion {
    pub fn server_config(&self) -> DevConfig {
        match self {
            KbVersion::G15 => DevConfig {
                dev_name: "046d:c227",
                interface: 0,
                configuration: 1,
                endpoint: 129,
                friendly_name: "G15",
            },
            KbVersion::G19 => DevConfig {
                dev_name: "046d:c229",
                interface: 0,
                configuration: 1,
                endpoint: 129,
                friendly_name: "G19",
            },
        }
    }
}
impl FromStr for KbVersion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "G15" => Ok(Self::G15),
            "G19" => Ok(Self::G19),
            _ => Err(format!(
                "[!] {s} is not a valid option: Choose between [G15, G19]"
            )),
        }
    }
}
