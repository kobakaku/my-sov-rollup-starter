use anyhow::{anyhow, bail, Context as _};
use sov_modules_api::{Context, StateMap, StateMapAccessor, WorkingSet};

use crate::{
    address::{CollectionAddress, CreatorAddress},
    utils::get_collection_address,
};

pub enum CollectionState<C: Context> {
    Frozen(Collection<C>),
    Mutable(MutableCollection<C>),
}

impl<C: Context> CollectionState<C> {
    pub fn get_mutable_or_bail(&self) -> anyhow::Result<MutableCollection<C>> {
        match self {
            CollectionState::Frozen(collection) => bail!(
                "Collection with name: {} , creator: {} is frozen",
                collection.collection_name,
                collection.creator
            ),
            CollectionState::Mutable(collection) => Ok(collection.clone()),
        }
    }
}

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq, Clone)]
pub struct Collection<C: Context> {
    pub collection_name: String,
    pub creator: CreatorAddress<C>,
    pub frozen: bool,
    pub supply: u64,
    pub collection_uri: String,
}

impl<C: Context> Collection<C> {
    pub(crate) fn new(
        collection_name: &str,
        collection_uri: &str,
        collections: &StateMap<CollectionAddress<C>, Collection<C>>,
        context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> anyhow::Result<(CollectionAddress<C>, Collection<C>)> {
        let creator = context.sender();
        let collection_address = get_collection_address(collection_name, creator.as_ref());
        let collection = collections.get(&collection_address, working_set);
        if collection.is_some() {
            Err(anyhow!(
                "Collection with name: {} already exsits create {}",
                collection_name,
                creator
            ))
        } else {
            Ok((
                collection_address,
                Collection {
                    collection_name: collection_name.to_string(),
                    creator: CreatorAddress::new(creator),
                    frozen: false,
                    supply: 0,
                    collection_uri: collection_uri.to_string(),
                },
            ))
        }
    }

    pub(crate) fn get_owned_collection(
        collection_name: &str,
        collections: &StateMap<CollectionAddress<C>, Collection<C>>,
        context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> anyhow::Result<(CollectionAddress<C>, CollectionState<C>)> {
        let creator = context.sender();
        let collection_address = get_collection_address(collection_name, creator.as_ref());
        let collection = collections.get(&collection_address, working_set);
        if let Some(collection) = collection {
            if collection.frozen {
                Ok((collection_address, CollectionState::Frozen(collection)))
            } else {
                Ok((
                    collection_address,
                    CollectionState::Mutable(MutableCollection(collection)),
                ))
            }
        } else {
            Err(anyhow!("Collection not found")).with_context(|| {
                format!(
                    "Collection with name: {} does not exist for creator {}",
                    collection_name, creator
                )
            })
        }
    }
}

#[derive(Clone)]
pub struct MutableCollection<C: Context>(Collection<C>);

impl<C: Context> MutableCollection<C> {
    pub(crate) fn inner(&self) -> &Collection<C> {
        &self.0
    }

    pub(crate) fn set_collection_uri(&mut self, collection_uri: &str) {
        self.0.collection_uri = collection_uri.to_string();
    }
}
