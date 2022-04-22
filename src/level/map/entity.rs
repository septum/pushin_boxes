use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum MapEntity {
    /// Wall
    W,
    /// Floor
    F,
    /// Zone
    Z,
    /// Box
    B,
    /// Box in Zone
    P,
}
