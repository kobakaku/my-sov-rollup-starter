use async_trait::async_trait;

use sov_db::ledger_db::LedgerDB;
use sov_mock_da::{MockDaConfig, MockDaService, MockDaSpec};
use sov_modules_api::{
    default_context::{DefaultContext, ZkDefaultContext},
    Address, Spec,
};
use sov_modules_rollup_blueprint::RollupBlueprint;
use sov_modules_stf_blueprint::{kernels::basic::BasicKernel, StfBlueprint};
use sov_prover_storage_manager::ProverStorageManager;
use sov_risc0_adapter::host::Risc0Host;
use sov_rollup_interface::zk::ZkvmHost;
use sov_state::{config::Config as StorageConfig, DefaultStorageSpec, Storage, ZkStorage};
use sov_stf_runner::{ParallelProverService, RollupConfig, RollupProverConfig};
use stf_starter::runtime::Runtime;

pub struct MockRollup {}

#[async_trait]
impl RollupBlueprint for MockRollup {
    /// Data Availability service.
    type DaService = MockDaService;
    /// A specification for the types used by a DA layer.
    type DaSpec = MockDaSpec;
    /// Data Availability config.
    type DaConfig = MockDaConfig;

    /// Host of a zkVM program.
    type Vm = Risc0Host<'static>;

    /// Context for Zero Knowledge environment.
    type ZkContext = ZkDefaultContext;
    /// Context for Native environment.
    type NativeContext = DefaultContext;

    /// Manager for the native storage lifecycle.
    type StorageManager = ProverStorageManager<MockDaSpec, DefaultStorageSpec>;

    /// Runtime for the Zero Knowledge environment.
    type ZkRuntime = Runtime<Self::ZkContext, Self::DaSpec>;
    /// Runtime for the Native environment.
    type NativeRuntime = Runtime<Self::NativeContext, Self::DaSpec>;

    /// The kernel for the native environment.
    type NativeKernel = BasicKernel<Self::NativeContext, Self::DaSpec>;
    /// The kernel for the Zero Knowledge environment.
    type ZkKernel = BasicKernel<Self::ZkContext, Self::DaSpec>;

    /// Prover service.
    type ProverService = ParallelProverService<
        <<Self::NativeContext as Spec>::Storage as Storage>::Root,
        <<Self::NativeContext as Spec>::Storage as Storage>::Witness,
        Self::DaService,
        Self::Vm,
        StfBlueprint<
            Self::ZkContext,
            Self::DaSpec,
            <Self::Vm as ZkvmHost>::Guest,
            Self::ZkRuntime,
            Self::ZkKernel,
        >,
    >;

    /// Creates RPC methods for the rollup.
    fn create_rpc_methods(
        &self,
        storage: &<Self::NativeContext as sov_modules_api::Spec>::Storage,
        ledger_db: &LedgerDB,
        da_service: &Self::DaService,
    ) -> Result<jsonrpsee::RpcModule<()>, anyhow::Error> {
        // TODO set the sequencer address
        let sequencer = Address::new([0; 32]);

        let rpc_methods = sov_modules_rollup_blueprint::register_rpc::<
            Self::NativeRuntime,
            Self::NativeContext,
            Self::DaService,
        >(storage, ledger_db, da_service, sequencer)?;

        Ok(rpc_methods)
    }

    /// Creates instance of [`DaService`].
    async fn create_da_service(
        &self,
        rollup_config: &RollupConfig<Self::DaConfig>,
    ) -> Self::DaService {
        MockDaService::new(rollup_config.da.sender_address)
    }

    /// Creates instance of [`ProverService`].
    async fn create_prover_service(
        &self,
        prover_config: RollupProverConfig,
        rollup_config: &RollupConfig<Self::DaConfig>,
        _da_service: &Self::DaService,
    ) -> Self::ProverService {
        let vm = Risc0Host::new(risc0_starter::MOCK_DA_ELF);
        let zk_stf = StfBlueprint::new();
        let zk_storage = ZkStorage::new();
        let da_verifier = Default::default();

        ParallelProverService::new_with_default_workers(
            vm,
            zk_stf,
            da_verifier,
            prover_config,
            zk_storage,
            rollup_config.prover_service,
        )
    }

    /// Creates instance of [`Self::StorageManager`].
    /// Panics if initialization fails.
    fn create_storage_manager(
        &self,
        rollup_config: &RollupConfig<Self::DaConfig>,
    ) -> Result<Self::StorageManager, anyhow::Error> {
        let storage_config = StorageConfig {
            path: rollup_config.storage.path.clone(),
        };
        ProverStorageManager::new(storage_config)
    }
}

impl sov_modules_rollup_blueprint::WalletBlueprint for MockRollup {}
