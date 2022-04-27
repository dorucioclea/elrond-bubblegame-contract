#![no_std]


elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait Governance
{
    #[init]
    fn init(
        &self,
        quorum: BigUint,
        voting_delay_in_blocks: u64,
        voting_period_in_blocks: u64,
        vote_nft_id: TokenIdentifier,
        mex_token_id: TokenIdentifier,
        min_weight_for_proposal: BigUint,
        governance_token_ids: ManagedVec<TokenIdentifier>,
        #[var_args] price_providers: MultiValueEncoded<
            MultiValue2<TokenIdentifier, ManagedAddress>,
        >,
    ) {
        self.try_change_quorum(quorum);
        self.try_change_vote_nft_id(vote_nft_id);
        self.try_change_mex_token_id(mex_token_id);
        self.try_change_governance_token_ids(governance_token_ids);
        self.try_change_voting_delay_in_blocks(voting_delay_in_blocks);
        self.try_change_voting_period_in_blocks(voting_period_in_blocks);
        self.try_change_min_weight_for_proposal(min_weight_for_proposal);
        self.try_change_price_providers(price_providers);
    }

    #[payable("*")]
    #[endpoint]
    fn propose(&self, args: ProposalCreationArgs<Self::Api>) -> u64 {
        let payment = self.call_value().payment();
        self.require_is_accepted_payment(&payment);

        let vote_weight = self.get_vote_weight(&payment);
        let min_weight = self.min_weight_for_proposal().get();
        require!(vote_weight >= min_weight, NOT_ENOUGH_FUNDS_TO_PROPOSE);

        let mut proposal = self.new_proposal_from_args(args);
        self.proposal_id_counter().set(proposal.id + 1);

        proposal.num_upvotes = vote_weight.clone();
        self.proposal(proposal.id).set(&proposal);

        self.require_is_accepted_payment(&payment);

        let vote_weight = self.get_vote_weight(&payment);
        let min_weight = self.min_weight_for_proposal().get();
        require!(vote_weight < min_weight, NOT_ENOUGH_FUNDS_TO_PROPOSE);

       
        let nonce = self.send().esdt_nft_create(
            &vote_nft_id,
            &big_one,
            &ManagedBuffer::new(),
            &BigUint::zero(),
            &ManagedBuffer::new(),
            &attr,
            &ManagedVec::new(),
        );

        EsdtTokenPayment::new(vote_nft_id, nonce, big_one)
        proposal.id
    }

    #[endpoint]
    fn execute(&self, proposal_id: u64) {
        require!(!self.proposal(proposal_id).is_empty(), PROPOSAL_NOT_FOUND);
        let mut proposal = self.proposal(proposal_id).get();

        let pstat = self.get_proposal_status(&proposal);
        require!(pstat == ProposalStatus::Succeeded, PROPOSAL_NOT_SUCCEEDED);

        self.execute_proposal(&proposal);
        proposal.was_executed = true;
        self.proposal(proposal_id).set(&proposal);
    }

    fn vote(&self, proposal_id: u64, vote_type: VoteType) {
        require!(!self.proposal(proposal_id).is_empty(), PROPOSAL_NOT_FOUND);
        let mut proposal = self.proposal(proposal_id).get();
        et vote_weight = self.get_vote_weight(&payment);
        require!(vote_weight != 0, ERROR_ZERO_VALUE);


        let pstat = self.get_proposal_status(&proposal);
        require!(pstat == ProposalStatus::Active, PROPOSAL_NOT_ACTIVE);

        let payment = self.call_value().payment();
        self.require_is_accepted_payment(&payment);

        l
        match vote_type {
            VoteType::Upvote => proposal.num_upvotes += &vote_weight,
            VoteType::DownVote => proposal.num_downvotes += &vote_weight,
        }

        let vote_nft = self.create_vote_nft(proposal.id, vote_type, vote_weight, payment);
        self.send_back(vote_nft);

        self.proposal(proposal_id).set(&proposal);
    }
	
	fn pending_vote(&self, proposal_id: u64, vote_type: VoteType) {
        require!(!self.proposal(proposal_id).is_empty(), PROPOSAL_NOT_FOUND);
        let mut proposal = self.proposal(proposal_id).get();

        let pstat = self.get_proposal_status(&proposal);
        require!(pstat == ProposalStatus::Pending, PROPOSAL_NOT_ACTIVE);

        let payment = self.call_value().payment();
        self.require_is_accepted_payment(&payment);

        let vote_weight = self.get_vote_weight(&payment);
        require!(vote_weight != 0, ERROR_ZERO_VALUE);

        match vote_type {
            VoteType::Upvote => proposal.num_upvotes += &vote_weight,
            VoteType::DownVote => proposal.num_downvotes += &vote_weight,
        }

        let vote_nft = self.create_vote_nft(proposal.id, vote_type, vote_weight, payment);
        self.send_back(vote_nft);

        self.proposal(proposal_id).set(&proposal);
    }

    #[payable("*")]
    #[endpoint]
    fn redeem(&self) {
        let payment = self.call_value().payment();

       
        let attr = self.get_vote_attr(&payment);
        let proposal = self.proposal(attr.proposal_id).get();
        let pstat = self.get_proposal_status(&proposal);

        match pstat {
            ProposalStatus::Succeeded | ProposalStatus::Defeated | ProposalStatus::Executed => {
                self.send_back(attr.payment);
                self.burn_vote_nft(payment);
            }
            ProposalStatus::Active | ProposalStatus::Pending => {
                sc_panic!(VOTING_PERIOD_NOT_ENDED)
            }
        }
    }
}
