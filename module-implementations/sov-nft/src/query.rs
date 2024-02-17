use jsonrpsee::core::RpcResult;

use sov_modules_api::macros::rpc_gen;
use sov_modules_api::{Context, StateMapAccessor, WorkingSet};

use crate::address::{CollectionAddress, CreatorAddress};
use crate::utils::get_collection_address;
use crate::NonFungibleToken;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
/// Response for `getOwner` method
pub struct OwnerResponse<C: Context> {
    /// Optional owner address
    pub owner: Option<C::Address>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(bound(
    serialize = "CreatorAddress<C>: serde::Serialize",
    deserialize = "CreatorAddress<C>: serde::Deserialize<'de>"
))]
pub struct CollectionResponse<C: Context> {
    pub name: String,
    pub creator: CreatorAddress<C>,
    pub frozen: bool,
    pub supply: u64,
    pub collection_uri: String,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(bound(
    serialize = "CollectionAddress<C>: serde::Serialize",
    deserialize = "CollectionAddress<C>: serde::Deserialize<'de>"
))]
pub struct CollectionAddressResponse<C: Context> {
    pub collection_address: CollectionAddress<C>,
}

#[rpc_gen(client, server, namespace = "nft")]
impl<C: sov_modules_api::Context> NonFungibleToken<C> {
    #[rpc_method(name = "getOwner")]
    pub fn get_owner(
        &self,
        token_id: u64,
        working_set: &mut WorkingSet<C>,
    ) -> RpcResult<OwnerResponse<C>> {
        Ok(OwnerResponse {
            owner: self.owners.get(&token_id, working_set),
        })
    }

    #[rpc_method(name = "getCollectionAddress")]
    /// Get the collection address
    pub fn get_collection_address(
        &self,
        creator: CreatorAddress<C>,
        collection_name: &str,
        _working_set: &mut WorkingSet<C>,
    ) -> RpcResult<CollectionAddressResponse<C>> {
        let ca = get_collection_address::<C>(collection_name, creator.as_ref());
        Ok(CollectionAddressResponse {
            collection_address: ca,
        })
    }

    #[rpc_method(name = "getCollection")]
    pub fn get_collection(
        &self,
        creator: CreatorAddress<C>,
        collection_name: &str,
        working_set: &mut WorkingSet<C>,
    ) -> RpcResult<CollectionResponse<C>> {
        let collection_address = get_collection_address(collection_name, creator.as_ref());
        let c = self
            .collections
            .get(&collection_address, working_set)
            .unwrap();
        Ok(CollectionResponse {
            name: c.collection_name,
            creator: c.creator,
            frozen: c.frozen,
            supply: c.supply,
            collection_uri: c.collection_uri,
        })
    }
}
