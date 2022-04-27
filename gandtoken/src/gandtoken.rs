#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const TOKEN_DECIMALS: usize = 18;

#[elrond_wasm::contract]
pub trait GandToken {
    
    #[init]
    fn init(&self) {
        self.total_supply().set(&BigUint::zero());
        self.token_distribute_ended().set(false);
    }
    
    #[callback]
    fn esdt_issue_callback(
        &self,
        caller: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        let (returned_tokens, token_identifier) = self.call_value().payment_token_pair();
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.issue_success_event(caller, &token_identifier, &returned_tokens);
                self.issue_token_id().set(&token_identifier);
            },
            ManagedAsyncCallResult::Err(message) => {
                
                self.issue_failure_event(caller, &message.err_msg);
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.send().direct_egld(caller, &returned_tokens, &[]);
                }
            },
        }
    }

    // #[only_owner]
    // #[endpoint(setLocalRoles)]
    // fn set_local_roles(&self) {
    //     require!(
    //         !self.issue_token_id().is_empty(),
    //         "token was still issued"
    //     );

    //     let roles = [EsdtLocalRole::Mint, EsdtLocalRole::Burn];
    //     self.send()
    //         .esdt_system_sc_proxy()
    //         .set_special_roles(
    //             &self.blockchain().get_sc_address(),
    //             &self.issue_token_id().get(),
    //             roles[..].iter().cloned(),
    //         )
    //         .async_call()
    //         .call_and_exit()
    // }

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueTokens)]
    fn issue_tokens(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        require!(
            self.issue_token_id().is_empty(),
            "token was already issued"
        );

        let totalSupply: BigUint = BigUint::from(1_000_000_000_000u64).mul(&BigUint::from(1_000_000_000_000_000_000u64));
        self.total_supply().set(&totalSupply);

        let issue_cost = self.call_value().egld_value();
        let caller = self.blockchain().get_caller();
        let initial_supply: &BigUint = &self.total_supply().get();

        self.issue_started_event(&caller, &token_ticker, initial_supply);

        self.send()
            .esdt_system_sc_proxy()
            .issue_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                initial_supply,
                FungibleTokenProperties {
                    num_decimals: TOKEN_DECIMALS,
                    can_freeze: false,
                    can_wipe: false,
                    can_pause: false,
                    can_mint: true,
                    can_burn: false,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: false,
                },
            )
            .async_call()
            .with_callback(self.callbacks().esdt_issue_callback(&caller))
            .call_and_exit()
    }

    #[only_owner]
    #[endpoint(distributeToken)]
    fn distribute_tokens(
        &self,
        game_address: ManagedAddress,
        dao_address: ManagedAddress,
        liquidity_address: ManagedAddress,
        staking_address: ManagedAddress,
        marketing_address: ManagedAddress,
        team_address: ManagedAddress
    ) { 
        require!(
            !self.issue_token_id().is_empty(),
            "token was still issued"
        );
        require!(!self.token_distribute_ended().get(), "token was already distributed");
        self.token_distribute_ended().set(true);

        let total_supply = self.total_supply().get();
        let token_id = &self.issue_token_id().get();
        let game_amount = self.total_supply().get().div(BigUint::from(100u32)).mul(BigUint::from(25u32));
        let dao_amount = self.total_supply().get().div(BigUint::from(100u32)).mul(BigUint::from(45u32));
        let liquidity_amount = self.total_supply().get().div(BigUint::from(100u32)).mul(BigUint::from(10u32));
        let staking_amount = self.total_supply().get().div(BigUint::from(100u32)).mul(BigUint::from(12u32));
        let marketing_amount = self.total_supply().get().div(BigUint::from(100u32)).mul(BigUint::from(4u32));
        let team_amount = total_supply.div(BigUint::from(100u32)).mul(BigUint::from(4u32));
        self.send().direct(&game_address, &token_id, 0, &game_amount, &[]);
        self.send().direct(&dao_address, &token_id, 0, &dao_amount, &[]);
        self.send().direct(&liquidity_address, &token_id, 0, &liquidity_amount, &[]);
        self.send().direct(&staking_address, &token_id, 0, &staking_amount, &[]);
        self.send().direct(&marketing_address, &token_id, 0, &marketing_amount, &[]);
        self.send().direct(&team_address, &token_id, 0, &team_amount, &[]);
    }

    #[view(getBalanceOf)]
    fn balance_of(&self, address: ManagedAddress) -> BigUint {
        self.blockchain()
            .get_esdt_balance(&address, &self.issue_token_id().get(), 0)
    }

    #[view(getTotalSupply)]
     #[storage_mapper("totalSupply")]
    fn total_supply(&self) -> SingleValueMapper<BigUint>;

    // storage

    #[view(getTokenDistributeEnded)]
    #[storage_mapper("tokenDistributeEnded")]
    fn token_distribute_ended(&self) -> SingleValueMapper<bool>;

    #[view(getIssueTokenId)]
    #[storage_mapper("issueTokenId")]
    fn issue_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
    // events

    #[event("issue-started")]
    fn issue_started_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] token_ticker: &ManagedBuffer,
        initial_supply: &BigUint,
    );

    #[event("issue-success")]
    fn issue_success_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] token_identifier: &TokenIdentifier,
        initial_supply: &BigUint,
    );

    #[event("issue-failure")]
    fn issue_failure_event(&self, #[indexed] caller: &ManagedAddress, message: &ManagedBuffer);

}
