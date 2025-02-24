use macaddr::MacAddr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct MacAddrSerde {
    pub mac_addr: MacAddr,
}

impl Serialize for MacAddrSerde {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for MacAddrSerde {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<MacAddrSerde>().map_err(serde::de::Error::custom)
    }
}

impl FromStr for MacAddrSerde {
    type Err = macaddr::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MacAddrSerde {
            mac_addr: MacAddr::from_str(&s)?,
        })
    }
}

impl Display for MacAddrSerde {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.mac_addr)
    }
}
