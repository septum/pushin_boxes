use bevy::reflect::TypeUuid;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(TypeUuid, Serialize, Deserialize, Clone)]
#[uuid = "2e5bbfc2-8dfd-4547-8c85-cbaf27533998"]
pub struct SaveFileData {
    /// Unlocked stock levels
    pub stock: Vec<usize>,
    /// Saved custom levels
    pub custom: HashMap<Uuid, usize>,
}

impl Default for SaveFileData {
    fn default() -> SaveFileData {
        SaveFileData {
            stock: vec![0],
            custom: HashMap::new(),
        }
    }
}

impl SaveFileData {
    pub fn new(stock: Vec<usize>, custom: HashMap<Uuid, usize>) -> SaveFileData {
        SaveFileData { stock, custom }
    }
}
