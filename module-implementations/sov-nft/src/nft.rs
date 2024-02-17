use sov_modules_api::Context;

use crate::address::{CollectionAddress, OwnerAddress};

pub type TokenId = u64;

pub struct NftIdentifier<C: Context>(pub TokenId, pub CollectionAddress<C>);

pub struct Nft<C: Context> {
    token_id: TokenId,
    collection_address: CollectionAddress<C>,
    owner: OwnerAddress<C>,
    frozen: bool,
    token_uri: String,
}
