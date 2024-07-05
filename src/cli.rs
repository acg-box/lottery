// std
use std::{fs::File, io::Write};
// crates.io
use anyhow::Result;
use clap::Parser;
use rand::{rngs::OsRng, seq::SliceRandom};
// self
use crate::bilibili::User;

#[derive(Debug, Parser)]
#[command(
	version = concat!(
		env!("CARGO_PKG_VERSION"),
		"-",
		env!("VERGEN_GIT_SHA"),
		"-",
		env!("VERGEN_CARGO_TARGET_TRIPLE"),
	),
	about,
	rename_all = "kebab",
)]
pub struct Cli {
	/// Path to the players list.
	#[arg(long, short, value_name = "PATH")]
	list: String,
}
impl Cli {
	pub fn run(&self) -> Result<()> {
		let players = serde_json::from_reader::<_, Vec<User>>(File::open(&self.list)?)?;
		let winner = players.choose(&mut OsRng).expect("no players found");

		tracing::info!("winner: {winner:?}");

		writeln!(
			File::create(format!(
				"{}-winner.json",
				self.list.split_once('.').expect("file name must end with dot").0
			))?,
			"{}",
			serde_json::to_string(winner)?,
		)?;

		Ok(())
	}
}
