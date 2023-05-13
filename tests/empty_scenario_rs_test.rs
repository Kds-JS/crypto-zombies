use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("file:output/crypto-zombies.wasm", crypto_zombies::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    multiversx_sc_scenario::run_rs("scenarios/empty.scen.json", world());
}
