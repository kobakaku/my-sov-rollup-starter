pub mod call;
pub mod genesis;
pub mod query;

use call::CallMessage;

use genesis::NonFungibleTokenConfig;
use sov_modules_api::{Error, Module, WorkingSet};

#[derive(sov_modules_api::ModuleInfo, Clone)]
pub struct NonFungibleToken<C: sov_modules_api::Context> {
    #[address]
    address: C::Address,

    #[state]
    admin: sov_modules_api::StateValue<C::Address>,

    #[state]
    owners: sov_modules_api::StateMap<u64, C::Address>,
    // If the module needs to refer to another module
    // #[module]
    // bank: sov_bank::Bank<C>,
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
            CallMessage::Mint { id } => self.mint(id, context, working_set),
            CallMessage::Transfer { to, id } => self.transfer(id, to, context, working_set),
            CallMessage::Burn { id } => self.burn(id, context, working_set),
        };
        Ok(call_result?)
    }
}
