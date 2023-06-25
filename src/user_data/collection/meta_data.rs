use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MetaData {
    pub name: String,
    pub description: String,
    pub pinned: bool,
    pub last_changed: String,
}
