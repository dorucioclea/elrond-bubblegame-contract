elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::{config, vote::VoteType};

#[derive(TypeAbi, TopEncode, TopDecode, PartialEq, Debug)]
pub enum ProposalStatus {
    Pending = 1,
    Active = 2,
    Canceled = 3,
    Defeated = 4,
    Succeeded = 5,
    Queued = 6,
    Expired = 7,
    Executed = 8,
}


#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, TypeAbi)]
pub struct Action<M: ManagedTypeApi> {
    pub gas_limit: u64,
    pub dest_address: ManagedAddress<M>,
    pub payments: ManagedVec<M, ManagedBuffer<M>>,
    pub endpoint_name: ManagedBuffer<M>,
    pub arguments: ManagedVec<M, ManagedBuffer<M>>,
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct ProposalCreationArgs<M: ManagedTypeApi> {
    pub description: ManagedBuffer<M>,
    pub actions: ManagedVec<M, Action<M>>,
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct Proposal<M: ManagedTypeApi>
{
    pub id: u64,
    pub creation_block: u64,
    pub proposer: ManagedAddress<M>,
    pub description: ManagedBuffer<M>,

    pub was_canceled: bool,
    pub was_executed: bool,
    pub actions: ManagedVec<M, Action<M>>,

    pub num_upvotes: BigUint<M>,
    pub num_downvotes: BigUint<M>,

    pub eta: u64,
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct Receipt<M: ManagedTypeApi>
{
   pub support: VoteType,
   pub votes: BigUint<M>,
}


#[elrond_wasm::module]
pub trait ProposalHelper:
    config::Config
{
    fn new_proposal_from_args(&self, args: ProposalCreationArgs<Self::Api>) -> Proposal<Self::Api> {
        Proposal {
            id: self.proposal_id_counter().get(),
            creation_block: self.blockchain().get_block_nonce(),
            proposer: self.blockchain().get_caller(),
            description: args.description,
            was_canceled: false,
            was_executed: false,
            actions: args.actions,
            num_upvotes: BigUint::zero(),
            num_downvotes: BigUint::zero(),
            eta: 0,
        }
    }

    fn get_proposal_status(&self, proposal: &Proposal<Self::Api>) -> ProposalStatus {
        if proposal.was_canceled {
            return ProposalStatus::Canceled;
        }
        if proposal.was_executed {
            return ProposalStatus::Executed;
        }

        let current_block = self.blockchain().get_block_nonce();
        let proposal_block = proposal.creation_block;
        let voting_delay = self.voting_delay_in_blocks().get();
        let voting_period = self.voting_period_in_blocks().get();

        let voting_start = proposal_block + voting_delay;
        let voting_end = voting_start + voting_period;

        if current_block < voting_start {
            return ProposalStatus::Pending;
        }
        if current_block >= voting_start && current_block < voting_end {
            return ProposalStatus::Active;
        }

        let total_upvotes = &proposal.num_upvotes;
        let total_downvotes = &proposal.num_downvotes;
        let quorum = &self.quorum().get();
        let timestamp = self.blockchain().get_block_timestamp();

        if total_upvotes > total_downvotes && total_upvotes >= quorum {
            if proposal.eta == 0 {
                return ProposalStatus::Succeeded;
            } else if proposal.was_executed {
                return ProposalStatus::Executed;
            } else if timestamp >= proposal.eta + self.grace_period().get() {
                return ProposalStatus::Expired;
            } else {
                return ProposalStatus::Queued;
            }
        } else {
            return ProposalStatus::Defeated;
        }
    }

    fn execute_proposal(&self, proposal: &Proposal<Self::Api>) {
        for action in proposal.actions.iter() {
            self.execute_action(&action).unwrap()
        }
    }

    fn execute_action(&self, action: &Action<Self::Api>) -> Result<(), &'static [u8]> {
        Self::Api::send_api_impl().direct_egld_execute(
            &action.dest_address,
            &BigUint::zero(),
            action.gas_limit,
            &action.endpoint_name,
            &ManagedArgBuffer::from(action.arguments.clone()),
        )
    }
}