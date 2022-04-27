#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const TOKEN_DECIMAL: u32 = 18;
const STAKE_MONTHS: [u64; 3] = [6, 12, 24];
const MONTHLY_REWARDS: [u64; 24] = [
    15_000_000_000,  12_900_000_000,  11_100_000_000,   9_600_000_000,
    8_200_000_000,   7_100_000_000,   6_100_000_000,    5_300_000_000,
    4_500_000_000,   3_900_000_000,   3_400_000_000,    2_900_000_000,
    2_500_000_000,   2_500_000_000,   2_500_000_000,    2_500_000_000,
    2_500_000_000,   2_500_000_000,   2_500_000_000,    2_500_000_000,
    2_500_000_000,   2_500_000_000,   2_500_000_000,    2_500_000_000
];

#[derive(TypeAbi, TopEncode, TopDecode, PartialEq, Debug)]
pub struct StakeInfo<M: ManagedTypeApi>{
    pub address: ManagedAddress<M>,
    pub stake_amount: BigUint<M>,
    pub stake_option: u32,
    pub lock_time: u64,
    pub unstake_time: u64,
    pub lock_month: usize
}

#[elrond_wasm::contract]
pub trait TokenStaking{
    #[init]
    fn init(
        &self,
        staking_token_id: TokenIdentifier,
        minimum_staking_amount: BigUint
    ) {
        self.staking_token_id().set(&staking_token_id);
        self.minimum_staking_amount().set(&minimum_staking_amount);
        self.staking_status().set(false);
        let zero = BigUint::zero();
        self.total_staking_amount().set(zero);

        let m: usize;
        for m in 0..24 {
            self.month_distributed(m).set(&BigUint::zero())
        }

    }

    #[only_owner]
    #[endpoint(startStaking)]
    fn start_staking(&self) {
        require!(!self.staking_status().get(), "Staking was already started");
        self.staking_status().set(true);
        let cur_time: u64 = self.blockchain().get_block_timestamp();
        self.staking_start_time().set(cur_time);
        self.staking_end_time().set(cur_time + 24 * 30 * 86400);
    }

    #[only_owner]
    #[endpoint(stopStaking)]
    fn stop_staking(&self) {
        require!(self.staking_status().get(), "Staking wasn't started");
        let cur_time: u64 = self.blockchain().get_block_timestamp();
        require!(cur_time < self.staking_end_time().get(), "You can't stop the stakeing in 24 months."); 
        self.staking_status().set(false);
    }

    #[endpoint(calcAPR)]
    fn calc_apr(&self) -> BigUint {
        require!(self.staking_status().get(), "The staking haven't started yet.");
        let cur_time: u64 = self.blockchain().get_block_timestamp();
        require!(cur_time < self.staking_end_time().get(), "The staking date has already passed"); 

        let cur_day = (cur_time - self.staking_start_time().get()) / 86400;
        
        let _month: usize = (cur_day / 30) as usize;
        let _day = cur_day % 30;
        let mut reward_amount = BigUint::zero();

        let mut m: usize;
        for m in 0.._month {          
            reward_amount = reward_amount.add(&BigUint::from(MONTHLY_REWARDS[m]));
        }

        let reward = BigUint::from(MONTHLY_REWARDS[_month]).div(30u32).mul(_day + 1);
        reward_amount = reward_amount.add(&reward).mul(&BigUint::from(1_000_000_000_000_000_000u64));
        let total_staking_amount = self.total_staking_amount().get();
        let apy: BigUint = reward_amount.mul(365u32).div(total_staking_amount).div(cur_day + 1);
        return apy;
    }

    #[payable("*")]    
    #[endpoint]
    fn stake(&self, stake_option: u32) {
        require!(self.staking_status().get(), "The staking haven't started yet.");
        let (payment_amount, payment_token) = self.call_value().payment_token_pair();
        let caller: ManagedAddress = self.blockchain().get_caller();

        require!(payment_token == self.staking_token_id().get(), "Invalid staking token");
        require!(stake_option < 3u32, "Invalid staking option");
        require!(self.staking_info(&caller).is_empty(), "You have already staked.");
        require!(payment_amount >= self.minimum_staking_amount().get(), "The staking amount must be greater than minimum amount.");

        let index: u32 = stake_option;        
        let stake_amount:BigUint = payment_amount.clone();
        let cur_time: u64 = self.blockchain().get_block_timestamp();
        let unstake_time = cur_time + (STAKE_MONTHS[index as usize] * 30 * 86400);
        let month: usize = ((cur_time - self.staking_start_time().get()) / (30 * 86400)) as usize;
        let mut staked_month = STAKE_MONTHS[index as usize] as usize;

        let stake_info = StakeInfo {
            address: self.blockchain().get_caller(),
            stake_amount: stake_amount,
            stake_option: stake_option,
            lock_time: cur_time,
            unstake_time: unstake_time,
            lock_month: month
        };

        let m: usize;
        if month + staked_month > 24 {
            staked_month = 24 - month;
        } 
        for m in 0..staked_month {
            self.month_distributed(m + month).update(|amount| *amount += payment_amount.clone());
        }

        self.staking_info(&self.blockchain().get_caller()).set(&stake_info);
        let total_stake_amount = self.total_staking_amount().get();
        self.total_staking_amount().set(total_stake_amount.add(&payment_amount));
    }

    #[endpoint]
    fn unstake(&self, stake_option: u32) {
        let caller: ManagedAddress = self.blockchain().get_caller();
        let cur_time: u64 = self.blockchain().get_block_timestamp();

        require!(!self.staking_info(&caller).is_empty(), "You didn't stake!");
        let stake_info = self.staking_info(&caller).get();
        require!(stake_info.unstake_time <= cur_time, "You can't unlock staking token yet.");

        let stake_token_id = self.staking_token_id().get();
        let unstake_amount: BigUint = stake_info.stake_amount;
        let month = stake_info.lock_month;
        
        let mut staked_month = STAKE_MONTHS[stake_info.stake_option as usize] as usize;
        let m: usize;
        let mut reward_tokens = BigUint::zero();

        if month + staked_month > 24 {
            staked_month = 24 - month;
        } 

        for m in 0..staked_month {
            let month_reward = BigUint::from(MONTHLY_REWARDS[m + month]);
            reward_tokens += month_reward * unstake_amount.clone() / self.month_distributed(m + month).get();
        }
        reward_tokens = reward_tokens.mul(&BigUint::from(1_000_000_000_000_000_000u64));
        reward_tokens += unstake_amount;
        self.send()
            .direct(&caller, &stake_token_id, 0, &reward_tokens, &[]);
               
        self.staking_info(&caller).clear();
    }
	
    #[view(getStakingTokenId)]
    #[storage_mapper("stakingTokenId")]
    fn staking_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getStakingStatus)]
    #[storage_mapper("stakingStatus")]
    fn staking_status(&self) -> SingleValueMapper<bool>;

    #[view(getStakingStartTime)]
    #[storage_mapper("stakingStartTime")]
    fn staking_start_time(&self) -> SingleValueMapper<u64>;

    #[view(getStakingEndTime)]
    #[storage_mapper("stakingEndTime")]
    fn staking_end_time(&self) -> SingleValueMapper<u64>;
    
    #[view(getMinimumStakingAmount)]
    #[storage_mapper("minimumStakingAmount")]
    fn minimum_staking_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getTotalStakingAmount)]
    #[storage_mapper("totalStakingAmount")]
    fn total_staking_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getStakingInfo)]
    #[storage_mapper("stakingInfo")]
    fn staking_info(&self, address: &ManagedAddress) -> SingleValueMapper<StakeInfo<Self::Api>>;

    #[storage_mapper("monthDistributed")]
    fn month_distributed(&self, month: usize) -> SingleValueMapper<BigUint>;

    #[event("stake")]
    fn stake_event(&self, #[indexed] user: &ManagedAddress, #[indexed] stake_amount: &BigUint, #[indexed] stake_option: u32);

    #[event("unstake")]
    fn unstake_event(&self, #[indexed] user: &ManagedAddress, #[indexed] unstake_amount: &BigUint);   
}
