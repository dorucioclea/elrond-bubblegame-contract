elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::vote::VoteType;

#[elrond_wasm::module]
pub trait Event {
    #[event("create-proposal")]
    fn create_proposal_event(&self, #[indexed] proposal_id: u64);

    #[event("cancel-proposal")]
    fn cancel_proposal_event(&self, #[indexed] proposal_id: u64);

    #[event("queue-proposal")]
    fn queue_proposal_event(&self, #[indexed] proposal_id: u64);

    #[event("execute-proposal")]
    fn execute_proposal_event(&self, #[indexed] proposal_id: u64);

    #[event("vote")]
    fn vote_event(&self, #[indexed] voter: &ManagedAddress, #[indexed] proposal_id: u64, vote_type: VoteType);
}
