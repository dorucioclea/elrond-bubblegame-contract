elrond_wasm::imports!();

use crate::errors::*;
use crate::proposal::*;

#[elrond_wasm::module]
pub trait Config {
    #[endpoint(changeQuorum)]
    fn change_quorum(&self, new_value: BigUint) {
        self.require_caller_self();

        self.try_change_quorum(new_value);
    }

    #[endpoint(changeMinWeightForProposing)]
    fn change_min_weight_for_proposal(&self, new_value: BigUint) {
        self.require_caller_self();

        self.try_change_min_weight_for_proposal(new_value);
    }

    #[endpoint(changeVotingDelayInBlocks)]
    fn change_voting_delay_in_blocks(&self, new_value: u64) {
        self.require_caller_self();

        self.try_change_voting_delay_in_blocks(new_value);
    }

    #[endpoint(changeTimelockDelay)]
    fn change_timelock_delay(&self, new_value: u64) {
        self.require_caller_self();

        self.try_change_timelock_delay(new_value);
    }

    #[endpoint(changeVotingPeriodInBlocks)]
    fn change_voting_period_in_blocks(&self, new_value: u64) {
        self.require_caller_self();

        self.try_change_voting_period_in_blocks(new_value);
    }


    fn require_caller_self(&self) {
        let caller = self.blockchain().get_caller();
        let sc_address = self.blockchain().get_sc_address();

        require!(caller == sc_address, INVALID_CALLER_NOT_SELF);
    }

    fn try_change_quorum(&self, new_value: BigUint) {
        require!(new_value != 0u64, ERROR_ZERO_VALUE);

        self.quorum().set(&new_value);
    }

    fn try_change_voting_delay_in_blocks(&self, new_value: u64) {
        require!(new_value != 0, ERROR_ZERO_VALUE);

        self.voting_delay_in_blocks().set(&new_value);
    }

    fn try_change_timelock_delay(&self, new_value: u64) {
        require!(new_value >= self.minimum_delay().get(), DELAY_EXCEED_MINIMUM);
        require!(new_value <= self.maximum_delay().get(), DELAY_NOT_EXCEED_MAXIMUM);

        self.timelock_delay().set(&new_value);
    }

    fn try_change_voting_period_in_blocks(&self, new_value: u64) {
        require!(new_value != 0, ERROR_ZERO_VALUE);

        self.voting_period_in_blocks().set(&new_value);
    }

    fn try_change_min_weight_for_proposal(&self, new_value: BigUint) {
        require!(new_value != 0u64, ERROR_ZERO_VALUE);

        self.min_weight_for_proposal().set(&new_value);
    }

    fn try_change_staking_provider(&self, new_value: ManagedAddress) {
        self.staking_provider().set(&new_value);
    }

    fn try_change_guardian(&self, new_value: ManagedAddress) {
        self.guardian().set(&new_value);
    }

    #[view(getStakingProvider)]
    #[storage_mapper("staking_provider")]
    fn staking_provider(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getMinWeightForProposal)]
    #[storage_mapper("minWeightForProposal")]
    fn min_weight_for_proposal(&self) -> SingleValueMapper<BigUint>;

    #[view(getVotingDelayInBlocks)]
    #[storage_mapper("votingDelayInBlocks")]
    fn voting_delay_in_blocks(&self) -> SingleValueMapper<u64>;

    #[view(getTimelockDelay)]
    #[storage_mapper("timelockDelay")]
    fn timelock_delay(&self) -> SingleValueMapper<u64>;

    #[view(getVotingPeriodInBlocks)]
    #[storage_mapper("votingPeriodInBlocks")]
    fn voting_period_in_blocks(&self) -> SingleValueMapper<u64>;

    #[view(getQuorum)]
    #[storage_mapper("quorum")]
    fn quorum(&self) -> SingleValueMapper<BigUint>;

    #[view(getProposal)]
    #[storage_mapper("proposal")]
    fn proposal(&self, id: u64) -> SingleValueMapper<Proposal<Self::Api>>;

    #[view(getProposalIdCounter)]
    #[storage_mapper("proposalIdCounter")]
    fn proposal_id_counter(&self) -> SingleValueMapper<u64>;

    #[view(getReceipt)]
    #[storage_mapper("receipt")]
    fn receipt(&self, proposal_id: u64, voter: ManagedAddress) -> SingleValueMapper<Receipt<Self::Api>>;

    #[view(getGuardian)]
    #[storage_mapper("guardian")]
    fn guardian(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getGracePeriod)]
    #[storage_mapper("gracePeriod")]
    fn grace_period(&self) -> SingleValueMapper<u64>;

    #[view(getMinimumDelay)]
    #[storage_mapper("minimumDelay")]
    fn minimum_delay(&self) -> SingleValueMapper<u64>;

    #[view(getMaximumDelay)]
    #[storage_mapper("maximumDelay")]
    fn maximum_delay(&self) -> SingleValueMapper<u64>;

}