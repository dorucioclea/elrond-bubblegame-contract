import logging
from argparse import ArgumentParser
from pathlib import Path

from erdpy import config
from erdpy.accounts import Account
from erdpy.contracts import SmartContract
from erdpy.environments import TestnetEnvironment
from erdpy.projects import ProjectRust
from erdpy.proxy import ElrondProxy

logger = logging.getLogger("examples")

logger.info("abcdf")

if __name__ == '__main__':
    parser = ArgumentParser()
    parser.add_argument("--proxy", help="Proxy URL", default=config.get_proxy())
    parser.add_argument("--contract", help="Existing contract address")
    parser.add_argument("--pem", help="PEM file", required=True)
    args = parser.parse_args()

    logging.basicConfig(level=logging.INFO)

    proxy = ElrondProxy(args.proxy)
    network = proxy.get_network_config()
    chain = network.chain_id
    gas_price = network.min_gas_price
    tx_version = network.min_tx_version

    environment = TestnetEnvironment(args.proxy)
    user = Account(pem_file=args.pem)

    project = ProjectRust(Path(__file__).parent.parent)
    bytecode = project.get_bytecode()

    contract = SmartContract(address=args.contract)

    def deploy_mint_flow():
        global contract

        contract = SmartContract(bytecode=bytecode)

        tx, address = environment.deploy_contract(
            contract=contract,
            owner=user,
            arguments=[],
            gas_price=gas_price,
            gas_limit=50000000,
            value=None,
            chain=chain,
            version=tx_version
        )

        logger.info("Tx hash: %s", tx)
        logger.info("Contract address: %s", address.bech32())

    def issue_token_flow() :
        environment.execute_contract(
            contract=contract,
            caller=user,
            function="issueTokens",
            arguments=["str:GANDTOKEN", "str:GAND", 1000000000000],
            gas_price=gas_price,
            gas_limit=600000000,
            value=50000000000000000,
            chain=chain,
            version=tx_version
        )
   
    user.sync_nonce(ElrondProxy(args.proxy))

    environment.run_flow(deploy_mint_flow)
    user.nonce += 1
    environment.run_flow(issue_token_flow)
    user.nonce += 1
