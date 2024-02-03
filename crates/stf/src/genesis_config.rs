use anyhow::{bail, Context as _, Error as AError};
use sov_accounts::AccountConfig;
use sov_bank::BankConfig;
use sov_sequencer_registry::SequencerConfig;
use sov_stf_runner::read_json_file;
use std::path::PathBuf;

use sov_modules_api::{Context, DaSpec};
use sov_modules_stf_blueprint::Runtime as RuntimeTrait;

use crate::runtime::{GenesisConfig, Runtime};

/// Paths to genesis files.
pub struct GenesisPaths {
    /// Accounts genesis path.
    pub accounts_genesis_path: PathBuf,
    /// Bank genesis path.
    pub bank_genesis_path: PathBuf,
    /// Sequencer Registry genesis path.
    pub sequencer_genesis_path: PathBuf,
}

/// Creates genesis configuration.
pub fn get_genesis_config<C, Da>(
    genesis_paths: &GenesisPaths,
) -> Result<<Runtime<C, Da> as RuntimeTrait<C, Da>>::GenesisConfig, AError>
where
    C: Context,
    Da: DaSpec,
{
    let genesis_config =
        create_genesis_cofig(genesis_paths).context("Unable to read genesis configuration")?;
    validate_config(genesis_config)
}

fn validate_config<C, Da>(
    genesis_config: GenesisConfig<C, Da>,
) -> Result<<Runtime<C, Da> as RuntimeTrait<C, Da>>::GenesisConfig, AError>
where
    C: Context,
    Da: DaSpec,
{
    let token_address = &sov_bank::get_genesis_token_address::<C>(
        &genesis_config.bank.tokens[0].token_name,
        genesis_config.bank.tokens[0].salt,
    );

    let coins_token_addr = &genesis_config
        .sequencer_registry
        .coins_to_lock
        .token_address;

    if coins_token_addr != token_address {
        bail!(
            "Wrong token address in `sequencer_registry_config` expected {} but found {}",
            token_address,
            coins_token_addr
        )
    }

    Ok(genesis_config)
}

fn create_genesis_cofig<C, Da>(genesis_paths: &GenesisPaths) -> Result<GenesisConfig<C, Da>, AError>
where
    C: Context,
    Da: DaSpec,
{
    let accounts_config: AccountConfig<C> = read_json_file(&genesis_paths.accounts_genesis_path)?;
    let bank_config: BankConfig<C> = read_json_file(&genesis_paths.bank_genesis_path)?;
    let sequencer_registry_config: SequencerConfig<C, Da> =
        read_json_file(&genesis_paths.sequencer_genesis_path)?;

    Ok(GenesisConfig::new(
        accounts_config,
        bank_config,
        sequencer_registry_config,
    ))
}
