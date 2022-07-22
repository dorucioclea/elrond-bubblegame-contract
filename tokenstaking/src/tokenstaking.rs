#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const TOKEN_DECIMAL: u32 = 18;
const STAKE_MONTHS: [u64; 4] = [6, 12, 18, 24];
const MONTHLY_REWARDS: [u64; 24] = [
    15_000_000_000,  12_900_000_000,  11_100_000_000,   9_600_000_000,
    8_200_000_000,   7_100_000_000,   6_100_000_000,    5_300_000_000,
    4_500_000_000,   3_900_000_000,   3_400_000_000,    2_900_000_000,
    2_500_000_000,   2_500_000_000,   2_500_000_000,    2_500_000_000,
    2_500_000_000,   2_500_000_000,   2_500_000_000,    2_500_000_000,
    2_500_000_000,   2_500_000_000,   2_500_000_000,    2_500_000_000
];

const POW_DECIMAL: u64 = 1_000_000_000_000_000_000;
const YEAR_DAYS: u64 = 365;
const MONTH_DAYS: u64 = 30;
const DAY_SECONDS: u64 = 86400;
const STAKING_PERIOD_DAYS: usize = 24 * 30;

#[derive(TypeAbi, TopEncode, TopDecode, PartialEq, Debug)]
pub struct StakeInfo<M: ManagedTypeApi>{
    pub address: ManagedAddress<M>,
    pub stake_amount: BigUint<M>,
    pub stake_option: usize,
    pub lock_time: u64,
    pub unlock_time: u64,
    pub last_claim_time: u64,
    pub from_day: usize,
    pub to_day: usize,
    pub last_claim_day: usize,
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

        let d: usize;
        for d in 0..STAKING_PERIOD_DAYS {
            self.day_distributed(d).set(&BigUint::zero())
        }

    }

    #[only_owner]
    #[endpoint(startStaking)]
    fn start_staking(&self) {
        require!(!self.staking_status().get(), "Staking was already started");
        self.staking_status().set(true);
        let cur_time: u64 = self.blockchain().get_block_timestamp();
        self.staking_start_time().set(cur_time);
        self.staking_end_time().set(cur_time + (STAKING_PERIOD_DAYS as u64) * DAY_SECONDS);
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
        
        let total_staking_amount = self.total_staking_amount().get();

        if total_staking_amount == BigUint::zero() {
            return BigUint::zero();
        }

        let cur_time: u64 = self.blockchain().get_block_timestamp();
        let mut cur_day: usize = ((cur_time - self.staking_start_time().get()) / DAY_SECONDS) as usize;
        if cur_day > STAKING_PERIOD_DAYS {
            cur_day = STAKING_PERIOD_DAYS;
        }

        if cur_day == 0 {
            return BigUint::zero();
        }

        

        let mut reward_amount = BigUint::zero();
        let d: usize;
        for d in 0..cur_day {
            let mut day_reward = BigUint::from(MONTHLY_REWARDS[d / (MONTH_DAYS as usize)]);
            day_reward = day_reward.mul(&BigUint::from(POW_DECIMAL));
            reward_amount += day_reward;
        }

        let apr: BigUint = reward_amount.mul(YEAR_DAYS).div(total_staking_amount).div(cur_day as u64);
        return apr;
    }


    #[payable("*")]    
    #[endpoint]
    fn stake(&self, stake_option: usize) {
        require!(self.staking_status().get(), "The staking haven't started yet.");
        let (payment_amount, payment_token) = self.call_value().payment_token_pair();
        let caller: ManagedAddress = self.blockchain().get_caller();

        require!(payment_token == self.staking_token_id().get(), "Invalid staking token");
        require!(stake_option < STAKE_MONTHS.len(), "Invalid staking option");
        require!(self.staking_info(&caller).is_empty(), "You have already staked.");
        require!(payment_amount >= self.minimum_staking_amount().get(), "The staking amount must be greater than minimum amount.");

        let stake_amount:BigUint = payment_amount.clone();
        let lock_time: u64 = self.blockchain().get_block_timestamp();
        let unlock_time = lock_time + (STAKE_MONTHS[stake_option] * MONTH_DAYS * DAY_SECONDS);
        let from_day: usize = ((lock_time - self.staking_start_time().get()) / DAY_SECONDS) as usize;
        let mut to_day = from_day + (STAKE_MONTHS[stake_option] * MONTH_DAYS) as usize;

        if to_day > STAKING_PERIOD_DAYS {
            to_day = STAKING_PERIOD_DAYS
        }

        let stake_info = StakeInfo {
            address: self.blockchain().get_caller(),
            stake_amount: stake_amount,
            stake_option: stake_option,
            lock_time: lock_time,
            unlock_time: unlock_time,
            last_claim_time: lock_time,
            from_day: from_day,
            to_day: to_day, 
            last_claim_day: from_day,
        };

        let d: usize; 
        for d in from_day..to_day {
            self.day_distributed(d).update(|amount| *amount += payment_amount.clone());
        }

        self.staking_info(&self.blockchain().get_caller()).set(&stake_info);
        self.stake_event(&caller, &payment_amount, stake_option);
        
        let total_stake_amount = self.total_staking_amount().get();
        self.total_staking_amount().set(total_stake_amount.add(&payment_amount));


    }

    #[endpoint]
    fn unstake(&self) {
        let caller: ManagedAddress = self.blockchain().get_caller();
        let cur_time: u64 = self.blockchain().get_block_timestamp();

        require!(!self.staking_info(&caller).is_empty(), "You didn't stake!");
        let stake_info = self.staking_info(&caller).get();
        require!(stake_info.unlock_time <= cur_time, "You can't unlock staking token yet.");

        let stake_token_id = self.staking_token_id().get();
        let stake_amount: BigUint = stake_info.stake_amount;

        self.send()
            .direct(&caller, &stake_token_id, 0, &stake_amount, &[]);
        self.unstake_event(&caller, &stake_amount);
               
        self.staking_info(&caller).clear();
    }

    #[endpoint]
    fn claim(&self) {
        let caller: ManagedAddress = self.blockchain().get_caller();
        let cur_time: u64 = self.blockchain().get_block_timestamp();

        require!(!self.staking_info(&caller).is_empty(), "You didn't stake!");
        let rewards = self.get_rewards(&caller);
        require!(rewards > 0, "You haven't claim amount");
        let stake_token_id = self.staking_token_id().get();
        let mut stake_info = self.staking_info(&caller).get();
        let to_day: usize = stake_info.to_day;
        let lock_time = stake_info.lock_time;
        let from_day = stake_info.from_day;

        let mut cur_day: usize = from_day + ((cur_time - lock_time) / DAY_SECONDS) as usize;
        if cur_day > to_day {
            cur_day = to_day;
        }
        
        self.send()
            .direct(&caller, &stake_token_id, 0, &rewards, &[]);
         self.claim_event(&caller, &rewards);
        
        stake_info.last_claim_time = cur_time;
        stake_info.last_claim_day = cur_day;
        self.staking_info(&caller).set(stake_info);

    } 

    #[view(getRewards)]
    fn get_rewards(&self, user: &ManagedAddress) -> BigUint {
        require!(!self.staking_info(&user).is_empty(), "You didn't stake!");

        let stake_info = self.staking_info(&user).get();
        let lock_time = stake_info.lock_time;
        let from_day = stake_info.from_day;
        let last_claim_day: usize = stake_info.last_claim_day;
        
        let to_day: usize = stake_info.to_day;
        let stake_amount = stake_info.stake_amount;

        let cur_time: u64 = self.blockchain().get_block_timestamp();
        let mut cur_day: usize = from_day + ((cur_time - lock_time) / DAY_SECONDS) as usize;
        if cur_day > to_day {
            cur_day = to_day;
        }
        
        let mut rewards = BigUint::zero();
        let d: usize;
        for d in last_claim_day..cur_day {
            let mut day_reward = BigUint::from(MONTHLY_REWARDS[d / (MONTH_DAYS as usize)]);
            day_reward = day_reward.mul(&BigUint::from(POW_DECIMAL));
            rewards += day_reward * stake_amount.clone() / self.day_distributed(d).get();
        }
        return rewards;
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

    #[storage_mapper("dayDistributed")]
    fn day_distributed(&self, day: usize) -> SingleValueMapper<BigUint>;

    #[event("stake")]
    fn stake_event(&self, #[indexed] user: &ManagedAddress, #[indexed] stake_amount: &BigUint, #[indexed] stake_option: usize);

    #[event("unstake")]
    fn unstake_event(&self, #[indexed] user: &ManagedAddress, #[indexed] unstake_amount: &BigUint);   

    #[event("claim")]
    fn claim_event(&self, #[indexed] user: &ManagedAddress, #[indexed] reward_amount: &BigUint);   
}