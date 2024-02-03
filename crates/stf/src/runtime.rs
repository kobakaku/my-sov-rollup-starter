#![allow(unused_doc_comments)]

#[cfg(feature = "native")]
pub use sov_accounts::{AccountsRpcImpl, AccountsRpcServer};
#[cfg(feature = "native")]
pub use sov_bank::{BankRpcImpl, BankRpcServer};
#[cfg(feature = "native")]
use sov_modules_api::Spec;
use sov_modules_api::{
    macros::{expose_rpc, CliWallet, DefaultRuntime},
    Context, DaSpec, DispatchCall, Genesis, MessageCodec,
};
#[cfg(feature = "native")]
pub use sov_sequencer_registry::{SequencerRegistryRpcImpl, SequencerRegistryRpcServer};

#[cfg(feature = "native")]
use crate::genesis_config::get_genesis_config;
#[cfg(feature = "native")]
use crate::genesis_config::GenesisPaths;

/// Runtime lifecycle:
///
/// 1. Initialization:
///     When a rollup is deployed for the first time, it needs to set its genesis state.
///     The `#[derive(Genesis)` macro will generate `Runtime::genesis(config)` method which returns
///     `Storage` with the initialized state.
///
/// 2. Calls:      
///     The `Module` interface defines a `call` method which accepts a module-defined type and triggers the specific `module logic.`
///     In general, the point of a call is to change the module state, but if the call throws an error,
///     no state is updated (the transaction is reverted).
///
/// `#[derive(MessageCodec)` adds deserialization capabilities to the `Runtime` (by implementing the `decode_call` method).
/// `Runtime::decode_call` accepts a serialized call message and returns a type that implements the `DispatchCall` trait.
///  The `DispatchCall` implementation (derived by a macro) forwards the message to the appropriate module and executes its `call` method.
#[cfg_attr(feature = "native", derive(CliWallet), expose_rpc)]
#[derive(Genesis, DispatchCall, MessageCodec, DefaultRuntime)]
#[serialization(borsh::BorshDeserialize, borsh::BorshSerialize)]
#[cfg_attr(feature = "serde", serialization(serde::Serialize, serde::Deserialize))]
pub struct Runtime<C: Context, Da: DaSpec> {
    /// The Accounts module.
    pub accounts: sov_accounts::Accounts<C>,
    /// The Bank module.
    pub bank: sov_bank::Bank<C>,
    /// The Sequencer Registry module.
    pub sequencer_registry: sov_sequencer_registry::SequencerRegistry<C, Da>,
}

impl<C, Da> sov_modules_stf_blueprint::Runtime<C, Da> for Runtime<C, Da>
where
    C: Context,
    Da: DaSpec,
{
    /// GenesisConfig type.
    type GenesisConfig = GenesisConfig<C, Da>;

    #[cfg(feature = "native")]
    /// GenesisPaths type.
    type GenesisPaths = GenesisPaths;

    #[cfg(feature = "native")]
    /// Default rpc methods.
    fn rpc_methods(storage: <C as Spec>::Storage) -> jsonrpsee::RpcModule<()> {
        get_rpc_methods::<C, Da>(storage)
    }

    #[cfg(feature = "native")]
    /// Reads genesis configs.
    fn genesis_config(
        genesis_paths: &Self::GenesisPaths,
    ) -> Result<Self::GenesisConfig, anyhow::Error> {
        get_genesis_config(genesis_paths)
    }
}
