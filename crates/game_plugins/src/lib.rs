#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::module_inception)]
#![cfg_attr(target_family = "wasm", allow(unused_mut))]
#![cfg_attr(target_family = "wasm", allow(unused_imports))]
#![cfg_attr(target_family = "wasm", allow(unused_variables))]
#![cfg_attr(target_family = "wasm", allow(dead_code))]

mod input;
mod level;
mod save_file;
mod state;

pub mod assets;
pub mod config;
pub mod scenes;
