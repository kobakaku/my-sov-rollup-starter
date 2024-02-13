use anyhow::bail;
use sov_modules_api::{StateMapAccessor, StateValueAccessor, WorkingSet};

use crate::NonFungibleToken;

#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize)
)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq, Clone)]
pub enum CallMessage<C: sov_modules_api::Context> {
    Mint {
        /// The id of new token. Caller is an owner
        id: u64,
    },
    Transfer {
        /// The address to which the token will be transferred.
        to: C::Address,
        /// The token id to transfer.
        id: u64,
    },
    Burn {
        id: u64,
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

    pub(crate) fn mint(
        &self,
        id: u64,
        context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> anyhow::Result<sov_modules_api::CallResponse> {
        if self.owners.get(&id, working_set).is_some() {
            bail!("Token with id {} already exists", id);
        }

        self.owners.set(&id, context.sender(), working_set);

        working_set.add_event("NFT mint", &format!("A token with id {id} was minted"));
        Ok(sov_modules_api::CallResponse::default())
    }

    pub(crate) fn transfer(
        &self,
        id: u64,
        to: C::Address,
        context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> anyhow::Result<sov_modules_api::CallResponse> {
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
        Ok(sov_modules_api::CallResponse::default())
    }

    pub(crate) fn burn(
        &self,
        id: u64,
        context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> anyhow::Result<sov_modules_api::CallResponse> {
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
        Ok(sov_modules_api::CallResponse::default())
    }
}
