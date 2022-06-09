mod handle;

use bevy::reflect::TypeUuid;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use handle::SaveFileHandle;

#[derive(Serialize, Deserialize, Clone)]
pub enum OptionKinds {
    OnOff(bool),
    Value(usize),
    Key(String)
}

#[derive(TypeUuid, Serialize, Deserialize, Clone)]
#[uuid = "2e5bbfc2-8dfd-4547-8c85-cbaf27533998"]
pub struct SaveFile {
    pub stock: Vec<usize>,
    pub custom: HashMap<Uuid, usize>,
    pub options: HashMap<String, OptionKinds>,
}

impl Default for SaveFile {
    fn default() -> SaveFile {
        SaveFile {
            stock: vec![0],
            custom: HashMap::new(),
            options: HashMap::new(),
        }
    }
}

impl SaveFile {
    pub fn new(
        stock: Vec<usize>,
        custom: HashMap<Uuid, usize>,
        options: HashMap<String, OptionKinds>,
    ) -> SaveFile {
        SaveFile {
            stock,
            custom,
            options,
        }
    }

    pub fn stock_levels_len(&self) -> usize {
        self.stock.len()
    }

    pub fn custom_levels_len(&self) -> usize {
        self.custom.len()
    }

    pub fn insert_stock_level_record(&mut self, moves: usize) {
        self.stock.push(moves);
    }

    pub fn insert_custom_level_record(&mut self, uuid: Uuid, moves: usize) {
        self.custom.insert(uuid, moves);
    }

    pub fn insert_option(&mut self, name: String, value: OptionKinds) {
        self.options.insert(name, value);
    }

    pub fn set_stock_level_record(&mut self, index: &usize, moves: usize) {
        self.stock[*index] = moves;
    }

    pub fn set_custom_level_record(&mut self, uuid: &Uuid, moves: usize) {
        *self.custom.get_mut(uuid).unwrap() = moves;
    }

    pub fn set_option(&mut self, name: &String, value: OptionKinds) {
        *self.options.get_mut(name).unwrap() = value;
    }

    pub fn get_stock_level_record(&self, index: &usize) -> usize {
        self.stock[*index]
    }

    pub fn get_custom_level_record(&self, uuid: &Uuid) -> usize {
        self.custom[uuid]
    }

    pub fn get_option(&self, name: &str) -> Option<OptionKinds> {
        self.options[name]
    }
}
