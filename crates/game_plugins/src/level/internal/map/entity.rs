use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub enum MapEntity {
    #[default]
    /// Floor
    F,
    /// Zone
    Z,
    /// Box in Floor
    B,
    /// Box in Zone
    P,
    /// Void
    V,
}
