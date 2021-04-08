use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct DndState {
    pub state: bool,
}
