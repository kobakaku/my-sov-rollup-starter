mod address;
mod call;
mod collection;
pub mod genesis;
mod nft;
pub mod query;
mod utils;

use address::CollectionAddress;
use call::CallMessage;
use collection::Collection;
use genesis::NonFungibleTokenConfig;

use nft::{Nft, NftIdentifier};
use sov_modules_api::{Error, Module, StateMap, WorkingSet};

#[derive(sov_modules_api::ModuleInfo)]
pub struct NonFungibleToken<C: sov_modules_api::Context> {
    #[address]
    address: C::Address,

    #[state]
    collections: StateMap<CollectionAddress<C>, Collection<C>>,

    #[state]
    nfts: StateMap<NftIdentifier<C>, Nft<C>>,

    #[state]
    admin: sov_modules_api::StateValue<C::Address>,

    #[state]
    owners: sov_modules_api::StateMap<u64, C::Address>,
}

impl<C: sov_modules_api::Context> Module for NonFungibleToken<C> {
    type Context = C;
    type Config = NonFungibleTokenConfig;
    type CallMessage = CallMessage<C>;
    type Event = ();

    // Rollupのデプロイ時に一度だけよびだされる
    fn genesis(&self, config: &Self::Config, working_set: &mut WorkingSet<C>) -> Result<(), Error> {
        Ok(self.init_module(config, working_set)?)
    }

    fn call(
        &self,
        msg: Self::CallMessage,
        context: &Self::Context,
        working_set: &mut WorkingSet<C>,
    ) -> Result<sov_modules_api::CallResponse, Error> {
        let call_result = match msg {
            CallMessage::CreateCollection {
                collection_name,
                collection_uri,
            } => self.create_collection(&collection_name, &collection_uri, context, working_set),
            CallMessage::UpdateCollection {
                collection_name,
                collection_uri,
            } => self.update_collection(&collection_name, &collection_uri, context, working_set),
            CallMessage::FreezeCollection { collection_name } => {
                self.freeze_collection(&collection_name, context, working_set)
            }
            CallMessage::MintNft {
                collection_name,
                token_id,
                owner,
                frozen,
                token_uri,
            } => self.mint_nft(
                &collection_name,
                &token_id,
                &owner,
                frozen,
                &token_uri,
                context,
                working_set,
            ),
            CallMessage::UpdateNft {
                collection_name,
                token_id,
                frozen,
                token_uri,
            } => self.update_nft(
                &collection_name,
                &token_id,
                frozen,
                token_uri,
                context,
                working_set,
            ),
            CallMessage::TransferNft {
                collection_address,
                token_id,
                to,
            } => self.transfer_nft(&collection_address, &token_id, &to, context, working_set),
            CallMessage::BurnNft {
                collection_name,
                token_id,
            } => self.burn_nft(&collection_name, &token_id, context, working_set),
        };
        Ok(call_result?)
    }
}
