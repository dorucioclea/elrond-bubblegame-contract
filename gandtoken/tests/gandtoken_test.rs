use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract_builder(
        "file:output/gandtoken-swap.wasm",
        gandtoken::ContractBuilder,
    );
    blockchain
}

#[test]
fn gandtoken_rs() {
    elrond_wasm_debug::mandos_rs("mandos/gandtoken.scen.json", world());
}
