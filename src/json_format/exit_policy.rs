use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExitPolicySummary {
    #[serde(default)]
    pub reject: Vec<String>,
    #[serde(default)]
    pub accept: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExitPolicyV6Summary {
    #[serde(default)]
    pub accept: Vec<String>,
    #[serde(default)]
    pub reject: Vec<String>,
}