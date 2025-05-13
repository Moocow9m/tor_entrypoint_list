use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relay {
    //pub nickname: String,
    //pub fingerprint: String,
    pub or_addresses: Vec<String>,
   /* pub last_seen: String,
    pub last_changed_address_or_port: String,
    pub first_seen: String,
    pub running: bool,
    pub flags: Vec<String>,
    pub country: String,
    pub country_name: String,
    #[serde(rename = "as")]
    pub as_field: String,
    pub as_name: Option<String>,
    pub consensus_weight: i64,
    #[serde(default)]
    pub verified_host_names: Vec<String>,
    pub last_restarted: String,
    pub bandwidth_rate: i64,
    pub bandwidth_burst: i64,
    pub observed_bandwidth: i64,
    pub advertised_bandwidth: i64,
    pub exit_policy: Vec<String>,
    pub exit_policy_summary: ExitPolicySummary,
    pub contact: Option<String>,
    pub platform: String,
    pub version: String,
    pub version_status: String,
    pub effective_family: Vec<String>,
    pub consensus_weight_fraction: f64,
    pub guard_probability: f64,
    pub middle_probability: f64,
    pub exit_probability: f64,
    pub recommended_version: bool,
    pub measured: bool,
    #[serde(default)]
    pub exit_addresses: Vec<String>,
    #[serde(default)]
    pub unverified_host_names: Vec<String>,
    #[serde(default)]
    pub alleged_family: Vec<String>,
    pub exit_policy_v6_summary: Option<ExitPolicyV6Summary>,
    #[serde(default)]
    pub indirect_family: Vec<String>,
    pub overload_general_timestamp: Option<i64>,*/
    pub dir_address: Option<String>,
}
