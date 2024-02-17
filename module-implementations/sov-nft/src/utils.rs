use sov_modules_api::digest::Digest;
use sov_modules_api::Context;

use crate::address::CollectionAddress;

pub(crate) fn get_collection_address<C: Context>(
    collection_name: &str,
    sender: &[u8],
) -> CollectionAddress<C> {
    let mut hasher = C::Hasher::new();
    hasher.update(sender);
    hasher.update(collection_name.as_bytes());

    let hash: [u8; 32] = hasher.finalize().into();
    CollectionAddress::new(&C::Address::from(hash))
}
