#![no_std]


elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const MONTHLY_REWARDS: [u64; 72] = [
    20_000_000_000,   18_404_000_000,   16_935_400_000,   15_584_000_000,   14_340_400_000,  13_196_000_000,   12_143_000_000,   11_174_000_000,   10_282_300_000,   9_461_800_000,
    8_706_700_000,    8_012_000_000,    7_372_600_000,    6_784_300_000,    6_242_900_000,
    5_744_700_000,    5_286_300_000,    4_864_500_000,    4_476_300_000,    4_119_100_000,
    3_790_400_000,    3_487_900_000,    3_209_600_000,    2_953_400_000,    2_717_800_000,
    2_500_900_000,    2_301_300_000,    2_117_700_000,    1_948_700_000,    1_793_200_000,
    1_650_100_000,    1_518_400_000,    1_397_200_000,    1_285_700_000,    1_183_100_000,
    1_088_700_000,    1_001_900_000,    921_900_000,    848_300_000,    780_600_000,
    718_300_000,    661_000_000,    608_300_000,    559_700_000,    515_100_000,
    474_000_000,    436_100_000,    401_300_000,    369_300_000,    339_800_000,
    312_700_000,    287_800_000,    264_800_000,    243_700_000,    224_200_000,
    206_300_000,    189_900_000,    174_700_000,    160_800_000,    147_900_000,
    136_100_000,    125_300_000,    115_300_000,    106_100_000,    97_600_000,
    89_800_000,    82_700_000,    76_100_000,    70_000_000,    64_400_000,
    59_300_000,    54_500_000
];

const PUBLIC_KEY: [u8; 33] = [
    0x02, 0x7b, 0x83, 0xad, 0x6a,0xfb, 0x12, 0x09, 
    0xf3, 0xc8, 0x2e,0xbe, 0xb0, 0x8c, 0x0c, 0x5f, 
    0xa9, 0xbf, 0x67, 0x24, 0x54, 0x85, 0x06, 0xf2, 
    0xfb, 0x4f, 0x99, 0x1e, 0x22, 0x87, 0xa7, 0x70, 0x90
];

const TOKEN_DECIMALS: u32 = 18;
const STKAE_MUL: [u16; 5]  = [12, 13, 14, 15, 11];
const NFT_MUL: [u16; 3] = [14, 17, 20];

#[derive(TypeAbi, TopEncode, TopDecode, PartialEq, Debug)]
pub struct StakeInfo<M: ManagedTypeApi>{
    pub address: ManagedAddress<M>,
    pub stake_amount: BigUint<M>,
    pub stake_option: u32,
    pub lock_time: u64,
}

#[elrond_wasm::contract]
pub trait Game {
    
    #[init]
    fn init(
        &self,
        game_token_id: TokenIdentifier
    ) {
        self.game_token_id().set(&game_token_id);
        self.game_status().set(false);
        self.game_start_time().set(0u64);
        self.game_end_time().set(0u64);
        self.whitelist_updated().set(0u16);
    }

    #[only_owner]
    #[endpoint(startGame)]
    fn start_game(&self) {
        require!(!self.game_status().get(), "Game has started already.");
        let cur_time: u64 = self.blockchain().get_block_timestamp();
        self.game_start_time().set(cur_time);
        self.game_end_time().set(cur_time + 72 * 30 * 86400);
        self.game_status().set(true);
    }
      
    #[endpoint(claimReward)]
    fn claim_reward(&self, reward_amount: BigUint, sig: &[u8]) {
        require!(self.game_status().get(), "Game hasn't started yet");
        let caller: ManagedAddress = self.blockchain().get_caller();
        let data1: [u8; 32] = caller.to_byte_array();

        let mut count = self.claim_count(&caller).get();
        count = count + 1u32;
        
        let data3: [u8; 4] = count.to_be_bytes();;

        let mut data2: ManagedBuffer = reward_amount.to_bytes_be_buffer();
        data2.append_bytes(&data1);
        data2.append_bytes(&data3);

        let hash: H256 = self.crypto().keccak256_legacy_alloc(data2.to_boxed_bytes().as_ref());
        let ok: bool = self.crypto().verify_custom_secp256k1(&PUBLIC_KEY, hash.as_bytes(), sig, MessageHashType::ECDSASha256);
        require!(ok, "Not signed by owner");

        let game_token_id = self.game_token_id().get(); 
        let total_amount = self.blockchain().get_sc_balance(&game_token_id, 0);
        require!(total_amount > reward_amount, "Insufficient amount");

        self.send()
        .direct(&caller, &game_token_id, 0, &reward_amount, &[]);

        self.claim_count(&caller).set(count);
        self.claim_event(&caller, &reward_amount);
    }

    #[endpoint]
    fn add(&self, pubKey: &[u8], msg: &[u8], sig: &[u8]) {
        let ok:bool = self.crypto().verify_custom_secp256k1(pubKey, msg, sig, MessageHashType::ECDSASha256);
        require!(ok, "Unverified");
        require!(!ok, "Verified");
    }
  
    #[view(getTimeReward)]
    fn time_reward(&self) -> u64 {
        let cur_time = self.blockchain().get_block_timestamp();
        if cur_time >= self.game_end_time().get() {
            return 0u64;
        }
        let delta = cur_time - self.game_start_time().get();
        let month = delta / (3600 * 24 * 30);
        return MONTHLY_REWARDS[month as usize];
    }

    #[view(getMultiplier)]
    fn multiplier(&self, address: ManagedAddress, nft: TokenIdentifier) -> u16 {
        let mut x: u16 = 1;    
        // if !self.staking_info(&address).is_empty() {
        //     let option = self.staking_info(&address).get().stake_option;
        //     x = x * STKAE_MUL[option as usize];
        // } else {
        //     x = x * 10u16;
        // }

        if !self.whitelist(&nft).is_empty() {
            let updated = self.whitelist_updated().get();
            let (option, _updated) = self.whitelist(&nft).get();
            if _updated == updated {
                x = x * NFT_MUL[option as usize];
            } else {
                x = x * 10u16;
            }
        } else {
            x = x * 10u16;
        }

        return x;
    }


    #[only_owner]
    #[endpoint]
    fn withdraw(&self, wallet: ManagedAddress, amount: BigUint) {
        let game_token_id = self.game_token_id().get();
        self.send()
            .direct(&wallet, &game_token_id, 0, &amount, &[]);
        self.withdraw_event(&wallet, &amount);
    }

    #[only_owner]
    #[endpoint(setWhitelist)]
    fn set_whitelist(&self, nfts: &[TokenIdentifier], c1: usize, c2: usize) {
        let mut updated = self.whitelist_updated().get();
        updated = updated + 1;
        self.whitelist_updated().set(updated);

        let i1 = c1;
        let i2 = c1 + c2;
        let len: usize = nfts.len();
        require!(i1 >= 0 && i2 >= i1 && len >= i2, "Invalid index");

        let mut i: usize;
        for i in 0..len {
            if i < i1 {
                self.whitelist(&nfts[i]).set((0u16, updated));
            } else if i < i2 {
                self.whitelist(&nfts[i]).set((1u16, updated));
            } else {
                self.whitelist(&nfts[i]).set((2u16, updated));
            }
        }
    }

    #[only_owner]
    #[endpoint(updateWhitelist)]
    fn update_whitelist(&self, nfts: &[TokenIdentifier], c1: usize, c2: usize) {
        let mut updated = self.whitelist_updated().get();
        let len: usize = nfts.len();
        
        let i1 = c1;
        let i2 = c1 + c2;
        let len: usize = nfts.len();
        require!(i1 >= 0 && i2 >= i1 && len >= i2, "Invalid index");

        let mut i: usize;
        for i in 0..len {
            if i < i1 {
                self.whitelist(&nfts[i]).set((0u16, updated));
            } else if i < i2 {
                self.whitelist(&nfts[i]).set((1u16, updated));
            } else {
                self.whitelist(&nfts[i]).set((2u16, updated));
            }
        }
    }

    #[event("claim")]
    fn claim_event(&self, #[indexed] user: &ManagedAddress, #[indexed] reward_amount: &BigUint);   

    #[event("withdraw")]
    fn withdraw_event(&self, #[indexed] user: &ManagedAddress, #[indexed] amount: &BigUint);   

    #[view(getGameTokenId)]
    #[storage_mapper("gameTokenId")]
    fn game_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getGameStatus)]
    #[storage_mapper("gameStatus")]
    fn game_status(&self) -> SingleValueMapper<bool>;

    #[view(getGameStartTime)]
    #[storage_mapper("gameStartTime")]
    fn game_start_time(&self) -> SingleValueMapper<u64>;

    #[view(getGameEndTime)]
    #[storage_mapper("gameEndTime")]
    fn game_end_time(&self) -> SingleValueMapper<u64>;

    #[view(getWhitelist)]
    #[storage_mapper("whitelist")]
    fn whitelist(&self, nft: &TokenIdentifier) -> SingleValueMapper<(u16, u16)>; 

    #[view(getClaimCount)]
    #[storage_mapper("claimCount")]
    fn claim_count(&self, addr: &ManagedAddress) -> SingleValueMapper<u32>;   

    #[view(getWhitelistUpdated)]
    #[storage_mapper("whitelistUpdated")]
    fn whitelist_updated(&self) -> SingleValueMapper<u16>;  
}
