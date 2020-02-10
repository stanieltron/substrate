use std::fmt::Debug;
use std::io::{Read, Write, Seek, self};
use std::fs;
use std::{str::FromStr, path::PathBuf};
use structopt::{StructOpt, clap::arg_enum};
use log::info;
use sc_network::{
	config::{build_multiaddr, NonReservedPeerMode, TransportConfig, NodeKeyConfig},
	multiaddr::Protocol,
};
use sc_service::{
	AbstractService, Configuration, ChainSpecExtension, RuntimeGenesis, ServiceBuilderCommand,
	config::{DatabaseConfig, KeystoreConfig}, ChainSpec, PruningMode,
};
use sp_runtime::traits::{Block as BlockT, Header as HeaderT};
use sp_runtime::generic::BlockId;

use crate::error;
use crate::runtime::run_until_exit;
use crate::execution_strategy::*;
use crate::execution_strategy::ExecutionStrategy;
use crate::commands::shared_params::SharedParams;
use crate::commands::node_key_params::NodeKeyParams;
use crate::commands::import_params::ImportParams;

/// The `check-block` command used to validate blocks.
#[derive(Debug, StructOpt, Clone)]
pub struct CheckBlockCmd {
	/// Block hash or number
	#[structopt(value_name = "HASH or NUMBER")]
	pub input: String,

	/// The default number of 64KB pages to ever allocate for Wasm execution.
	///
	/// Don't alter this unless you know what you're doing.
	#[structopt(long = "default-heap-pages", value_name = "COUNT")]
	pub default_heap_pages: Option<u32>,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub shared_params: SharedParams,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub import_params: ImportParams,
}

impl CheckBlockCmd {
	/// Run the check-block command
	pub fn run<G, E, B, BC, BB>(
		self,
		mut config: Configuration<G, E>,
		builder: B,
	) -> error::Result<()>
	where
		B: FnOnce(Configuration<G, E>) -> Result<BC, sc_service::error::Error>,
		G: RuntimeGenesis,
		E: ChainSpecExtension,
		BC: ServiceBuilderCommand<Block = BB> + Unpin,
		BB: sp_runtime::traits::Block + Debug,
		<<<BB as BlockT>::Header as HeaderT>::Number as std::str::FromStr>::Err: std::fmt::Debug,
		<BB as BlockT>::Hash: std::str::FromStr,
	{
		assert!(config.chain_spec.is_some(), "chain_spec must be present before continuing");

		self.import_params.update_config(
			&mut config,
			sc_service::Roles::FULL,
			self.shared_params.dev,
		)?;
		config.use_in_memory_keystore()?;

		let input = if self.input.starts_with("0x") { &self.input[2..] } else { &self.input[..] };
		let block_id = match FromStr::from_str(input) {
			Ok(hash) => BlockId::hash(hash),
			Err(_) => match self.input.parse::<u32>() {
				Ok(n) => BlockId::number((n as u32).into()),
				Err(_) => return Err(error::Error::Input("Invalid hash or number specified".into())),
			}
		};

		let start = std::time::Instant::now();
		run_until_exit(config, |config| {
			Ok(builder(config)?.check_block(block_id))
		})?;
		println!("Completed in {} ms.", start.elapsed().as_millis());

		Ok(())
	}
}
