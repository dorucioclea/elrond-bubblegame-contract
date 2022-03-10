#![no_std]

elrond_wasm::imports!();

const ADDITIONAL_AMOUNT_TO_CREATE: u64 = 1;
const BURN_TOKENS_GAS_LIMIT: u64 = 5000000;
const ADD_QUANTITY_GAS_LIMIT: u64 = 5000000;

#[elrond_wasm::moudle]
pub trait Mint {
    fn create_and_send_locked_assets(
        &self,
        amount: &Self::BigUint,
        attributes: &LockedTokenAttributes,
        address: &Address,
    ) -> Nonce {
        let token_id = self.locked_asset_token_id().get();
        self.create_tokens(&token_id, amount, attributes);
        let last_created_nonce = self.locked_asset_token_nonce().get();
        self.send()
            .transfer_tokens(&token_id, last_created_nonce, amount, address);
        last_created_nonce
    }

    fn create_tokens(
        &self,
        token: &TokenIdentifier,
        amount: &Self::BigUint,
        attributes: &LockedTokenAttributes,
    ) {
        let amount_to_create = amount + &Self::BigUint::from(ADDITIONAL_AMOUNT_TO_CREATE);
        self.send().esdt_nft_create::<LockedTokenAttributes>(
            self.blockchain().get_gas_left(),
            token.as_esdt_identifier(),
            &amount_to_create,
            &BoxedBytes::empty(),
            &Self::BigUint::zero(),
            &H256::zero(),
            attributes,
            &[BoxedBytes::empty()],
        );
        self.increase_nonce();
    }

    fn burn_locked_assets(&self, token_id: &TokenIdentifier, amount: &Self::BigUint, nonce: Nonce) {
        self.send()
            .burn_tokens(token_id, nonce, amount, BURN_TOKENS_GAS_LIMIT);
    }

    fn get_attributes(
        &self,
        token_id: &TokenIdentifier,
        token_nonce: Nonce,
    ) -> SCResult<LockedTokenAttributes> {
        let token_info = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address(),
            token_id.as_esdt_identifier(),
            token_nonce,
        );

        let attributes = token_info.decode_attributes::<LockedTokenAttributes>();
        match attributes {
            Result::Ok(decoded_obj) => Ok(decoded_obj),
            Result::Err(_) => {
                return sc_error!("Decoding error");
            }
        }
    }
}
