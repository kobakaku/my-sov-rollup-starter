use anyhow::bail;
use sov_modules_api::{CallResponse, StateMapAccessor, WorkingSet};

use crate::{
    address::{CollectionAddress, OwnerAddress, UserAddress},
    collection::Collection,
    NonFungibleToken,
};

#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize)
)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq, Clone)]
pub enum CallMessage<C: sov_modules_api::Context> {
    CreateCollection {
        collection_name: String,
        collection_uri: String,
    },
    UpdateCollection {
        collection_name: String,
        collection_uri: String,
    },
    FreezeCollection {
        collection_name: String,
    },
    MintNft {
        collection_name: String,
        token_id: String,
        owner: OwnerAddress<C>,
        frozen: bool,
        token_uri: String,
    },
    UpdateNft {
        collection_name: String,
        token_id: String,
        frozen: Option<bool>,
        token_uri: Option<String>,
    },
    TransferNft {
        collection_address: CollectionAddress<C>,
        token_id: String,
        to: UserAddress<C>,
    },
    BurnNft {
        collection_name: String,
        token_id: String,
    },
}

impl<C: sov_modules_api::Context> NonFungibleToken<C> {
    pub(crate) fn init_module(
        &self,
        _config: &<Self as sov_modules_api::Module>::Config,
        _working_set: &mut WorkingSet<C>,
    ) -> anyhow::Result<()> {
        // self.admin.set(&config.admin, working_set);
        // for (id, owner) in config.owners.iter() {
        //     if self.owners.get(id, working_set).is_some() {
        //         anyhow::bail!("Token id {} already exists", id);
        //     }
        //     self.owners.set(id, owner, working_set);
        //
        Ok(())
    }

    pub(crate) fn create_collection(
        &self,
        collection_name: &str,
        collection_uri: &str,
        context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> anyhow::Result<CallResponse> {
        let (collection_address, collection) = Collection::new(
            collection_name,
            collection_uri,
            &self.collections,
            context,
            working_set,
        )?;
        self.collections
            .set(&collection_address, &collection, working_set);
        Ok(CallResponse::default())
    }

    pub(crate) fn update_collection(
        &self,
        collection_name: &str,
        collection_uri: &str,
        context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> anyhow::Result<CallResponse> {
        let (collection_address, collection_state) = Collection::get_owned_collection(
            collection_name,
            &self.collections,
            context,
            working_set,
        )?;
        let mut collection = collection_state.get_mutable_or_bail()?;
        collection.set_collection_uri(collection_uri);
        self.collections
            .set(&collection_address, collection.inner(), working_set);
        Ok(CallResponse::default())
    }

    pub(crate) fn freeze_collection(
        &self,
        collection_name: &str,
        context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> anyhow::Result<CallResponse> {
        Ok(CallResponse::default())
    }

    pub(crate) fn mint_nft(
        &self,
        collection_name: &str,
        token_id: &str,
        owner: &OwnerAddress<C>,
        frozen: bool,
        token_uri: &str,
        context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> anyhow::Result<CallResponse> {
        Ok(CallResponse::default())
    }

    pub(crate) fn update_nft(
        &self,
        collection_name: &str,
        token_id: &str,
        frozen: Option<bool>,
        token_uri: Option<String>,
        context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> anyhow::Result<CallResponse> {
        Ok(CallResponse::default())
    }

    pub(crate) fn transfer_nft(
        &self,
        collection_address: &CollectionAddress<C>,
        token_id: &str,
        to: &UserAddress<C>,
        context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> anyhow::Result<CallResponse> {
        Ok(CallResponse::default())
    }

    pub(crate) fn burn_nft(
        &self,
        collection_name: &str,
        token_id: &str,
        context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> anyhow::Result<CallResponse> {
        Ok(CallResponse::default())
    }

    pub(crate) fn mint(
        &self,
        id: u64,
        context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> anyhow::Result<CallResponse> {
        if self.owners.get(&id, working_set).is_some() {
            bail!("Token with id {} already exists", id);
        }

        self.owners.set(&id, context.sender(), working_set);

        working_set.add_event("NFT mint", &format!("A token with id {id} was minted"));
        Ok(CallResponse::default())
    }

    pub(crate) fn transfer(
        &self,
        id: u64,
        to: C::Address,
        context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> anyhow::Result<CallResponse> {
        let token_owner = match self.owners.get(&id, working_set) {
            None => {
                anyhow::bail!("Token with id {} does not exist", id);
            }
            Some(owner) => owner,
        };
        if &token_owner != context.sender() {
            anyhow::bail!("Only token owner can transfer token");
        }
        self.owners.set(&id, &to, working_set);
        working_set.add_event(
            "NFT transfer",
            &format!("A token with id {id} was transferred"),
        );
        Ok(CallResponse::default())
    }

    pub(crate) fn burn(
        &self,
        id: u64,
        context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> anyhow::Result<CallResponse> {
        let token_owner = match self.owners.get(&id, working_set) {
            None => {
                anyhow::bail!("Token with id {} does not exist", id);
            }
            Some(owner) => owner,
        };
        if &token_owner != context.sender() {
            anyhow::bail!("Only token owner can burn token");
        }
        self.owners.remove(&id, working_set);

        working_set.add_event("NFT burn", &format!("A token with id {id} was burned"));
        Ok(CallResponse::default())
    }
}