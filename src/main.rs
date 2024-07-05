//! Lottery service of [acg.box](https://acg.box).

#![deny(clippy::all, missing_docs, unused_crate_dependencies)]

mod bilibili;
mod cli;

fn main() -> anyhow::Result<()> {
	color_eyre::install().unwrap();
	tracing_subscriber::fmt::init();
	<cli::Cli as clap::Parser>::parse().run()?;

	Ok(())
}
