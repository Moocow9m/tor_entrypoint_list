use serde::{Deserialize, Serialize};
use crate::json_format::bridge::Bridge;
use crate::json_format::relay::Relay;

pub mod relay;
pub mod exit_policy;
pub mod bridge;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TorDetails {
    //pub version: String,
    //pub build_revision: String,
    //pub relays_published: String,
    pub relays: Vec<Relay>,
    //pub bridges_published: String,
    pub bridges: Vec<Bridge>,
}