use crate::u_client::UnifiClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct AppState {
    pub on: Bandwidth,
    pub off: Bandwidth,
    pub client: Arc<UnifiClient>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct DndState {
    pub state: bool,
}

impl DndState {
    pub fn new(state: bool) -> Self {
        Self { state }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Bandwidth {
    pub(crate) down: i32,
    pub(crate) up: i32,
}

#[derive(Serialize)]
pub struct Auth {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Deserialize)]
pub struct ClientGroupsResponse {
    pub data: Vec<ClientGroupResponseData>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ClientGroupResponseData {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    #[serde(rename = "qos_rate_max_down")]
    pub max_down: i32,
    #[serde(rename = "qos_rate_max_up")]
    pub max_up: i32,
    pub site_id: String,
}

impl From<&ClientGroupResponseData> for Rate {
    fn from(d: &ClientGroupResponseData) -> Self {
        Self {
            max_down: d.max_down,
            max_up: d.max_up,
        }
    }
}

impl From<ClientGroupResponseData> for Rate {
    fn from(d: ClientGroupResponseData) -> Self {
        Self {
            max_down: d.max_down,
            max_up: d.max_up,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct Rate {
    pub max_down: i32,
    pub max_up: i32,
}
