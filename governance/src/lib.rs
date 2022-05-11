elrond_wasm::imports!();

use crate::config;

mod staking_proxy {
    elrond_wasm::imports!();
    #[elrond_wasm::proxy]
    pub trait StakingProxy {
        #[view(getVotingPower)]
        fn voting_power(&self, caller: &ManagedAddress<Self::Api>) -> BigUint;
    }
}

#[elrond_wasm::module]
pub trait Lib: config::Config {
    fn get_vote_weight(&self, caller: &ManagedAddress<Self::Api>) -> BigUint {
        let provider = self.staking_provider().get();
        let vote_weight = self
            .staking_proxy(provider)
            .voting_power(caller)
            .execute_on_dest_context();
        return vote_weight;
    }    

    #[proxy]
    fn staking_proxy(&self, to: ManagedAddress) -> staking_proxy::Proxy<Self::Api>;
}