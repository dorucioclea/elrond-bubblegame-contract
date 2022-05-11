#![no_std]
#![allow(clippy::type_complexity)]
#![feature(generic_associated_types)]

use proposal::ProposalCreationArgs;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod errors;
pub mod config;
pub mod proposal;
mod lib;
pub mod vote;
pub mod event;

use crate::proposal::*;
use crate::errors::*;
use crate::vote::*;


#[elrond_wasm::contract]
pub trait Governance:
    config::Config + lib::Lib + proposal::ProposalHelper + event::Event
{
    #[init]
    fn init(
        &self,
        quorum: BigUint,
        voting_delay_in_blocks: u64,
        voting_period_in_blocks: u64,
        min_weight_for_proposal: BigUint,
        guardian: ManagedAddress,
        staking_provider: ManagedAddress,
        grace_period: u64,
        timelock_delay: u64,
                
    ) {
        self.try_change_quorum(quorum);
        self.try_change_voting_delay_in_blocks(voting_delay_in_blocks);
        self.try_change_voting_period_in_blocks(voting_period_in_blocks);
        self.try_change_min_weight_for_proposal(min_weight_for_proposal);
        self.try_change_guardian(guardian);
        self.try_change_staking_provider(staking_provider);
        self.timelock_delay().set(timelock_delay);
        self.grace_period().set(grace_period);
    }

    #[endpoint]
    fn propose(&self, args: ProposalCreationArgs<Self::Api>) -> u64 {
        let caller = self.blockchain().get_caller();

        let vote_weight = self.get_vote_weight(&caller);
        let min_weight = self.min_weight_for_proposal().get();
        require!(vote_weight >= min_weight, NOT_ENOUGH_WEIGHT_TO_PROPOSE);

        let proposal = self.new_proposal_from_args(args);
        self.proposal_id_counter().set(proposal.id + 1);

        self.proposal(proposal.id).set(&proposal);
        self.create_proposal_event(proposal.id);
        proposal.id
    }
    
    #[endpoint]
    fn upvote(&self, proposal_id: u64) {
        self.vote(proposal_id, VoteType::Upvote);
    }

    #[endpoint]
    fn downvote(&self, proposal_id: u64) {
        self.vote(proposal_id, VoteType::DownVote);
    }

    #[view]
    fn state(&self, proposal_id: u64) -> ProposalStatus{
        require!(!self.proposal(proposal_id).is_empty(), PROPOSAL_NOT_FOUND);
        let proposal = self.proposal(proposal_id).get();
        let pstat = self.get_proposal_status(&proposal);
        return pstat;
    }

    #[endpoint]
    fn execute(&self, proposal_id: u64) {
        require!(!self.proposal(proposal_id).is_empty(), PROPOSAL_NOT_FOUND);
        let mut proposal = self.proposal(proposal_id).get();
        let timestamp = self.blockchain().get_block_timestamp();

        let pstat = self.get_proposal_status(&proposal);
        require!(pstat == ProposalStatus::Queued, PROPOSAL_NOT_QUEUED);
        require!(timestamp >= proposal.eta && timestamp < proposal.eta + self.grace_period().get(), NOT_SURPASSED_TIME_LOCK);

        proposal.was_executed = true;
        self.execute_proposal(&proposal);
        self.proposal(proposal_id).set(&proposal);

        self.execute_proposal_event(proposal.id);
    }

    #[endpoint]
    fn queue(&self, proposal_id: u64) {
        require!(!self.proposal(proposal_id).is_empty(), PROPOSAL_NOT_FOUND);
        let mut proposal = self.proposal(proposal_id).get();

        let pstat = self.get_proposal_status(&proposal);
        require!(pstat == ProposalStatus::Succeeded, PROPOSAL_NOT_SUCCEEDED);

        proposal.eta = self.blockchain().get_block_timestamp() + self.timelock_delay().get();
        self.proposal(proposal_id).set(&proposal);

        self.queue_proposal_event(proposal_id);
    }
    
    #[endpoint]
    fn cancel(&self, proposal_id: u64) {
        require!(!self.proposal(proposal_id).is_empty(), PROPOSAL_NOT_FOUND);

        let mut proposal = self.proposal(proposal_id).get();
        let pstat = self.get_proposal_status(&proposal);
        let caller = self.blockchain().get_caller();

        require!(pstat != ProposalStatus::Executed, PROPOSAL_NOT_CANCEL);
        require!(caller == self.guardian().get(), CALLER_NOT_GUARDIAN);

        proposal.was_canceled = true;
        self.proposal(proposal_id).set(&proposal);

        self.cancel_proposal_event(proposal_id);
    }

    #[endpoint]
    fn vote(&self, proposal_id: u64, vote_type: VoteType) {
        let voter = self.blockchain().get_caller();

        require!(!self.proposal(proposal_id).is_empty(), PROPOSAL_NOT_FOUND);
        require!(self.receipt(proposal_id, voter.clone()).is_empty(), ALREADY_VOTED);

        let mut proposal = self.proposal(proposal_id).get();
        let pstat = self.get_proposal_status(&proposal);
        require!(pstat == ProposalStatus::Active, PROPOSAL_NOT_ACTIVE);

        
        let vote_weight = self.get_vote_weight(&voter);

        require!(vote_weight != 0, ERROR_ZERO_VALUE);

        match vote_type {
            VoteType::Upvote => proposal.num_upvotes += &vote_weight,
            VoteType::DownVote => proposal.num_downvotes += &vote_weight,
        }

        self.proposal(proposal_id).set(&proposal);

        let receipt = Receipt {
            support: vote_type,
            votes: vote_weight,
        };
        self.receipt(proposal_id, voter).set(&receipt);
    }
}
