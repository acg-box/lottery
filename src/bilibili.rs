// crates.io
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
	pub id: u64,
	pub name: String,
}
