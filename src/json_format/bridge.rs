use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bridge {
    //pub nickname: String,
   // pub hashed_fingerprint: String,
    pub or_addresses: Vec<String>,
    /*pub last_seen: String,
    pub first_seen: String,
    pub running: bool,
    pub flags: Vec<String>,
    pub last_restarted: String,
    pub advertised_bandwidth: i64,
    pub platform: String,
    pub version: String,
    pub version_status: String,
    pub recommended_version: bool,
    pub bridgedb_distributor: Option<String>,
    pub contact: Option<String>,
    #[serde(default)]
    pub transports: Vec<String>,
    #[serde(default)]
    pub blocklist: Vec<String>,
    pub overload_general_timestamp: Option<i64>,*/
}