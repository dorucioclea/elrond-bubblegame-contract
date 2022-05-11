import { ProxyNetworkProvider  } from '@elrondnetwork/erdjs-network-providers';
import { SmartContract, Code, CodeMetadata, AbiRegistry, SmartContractAbi, BigUIntValue, U64Value,
    ResultsParser, Interaction, Transaction, Account, Address, ContractFunction, Struct, 
    StructType, Field, BytesValue, List, ListType, BytesType, EnumType, EnumValue, U8Value, TransactionWatcher, AddressValue } from '@elrondnetwork/erdjs';
import { UserSigner, UserSecretKey } from '@elrondnetwork/erdjs-walletcore';
import { promises, readFileSync, writeFileSync } from 'fs';
import { isConstructorDeclaration } from 'typescript';

let networkProvider = new ProxyNetworkProvider("https://devnet-gateway.elrond.com", { timeout: 5000 });

let pemFileContent = readFileSync("wallet-owner.pem", { encoding: "utf8" }).trim();
let signer = new UserSigner(UserSecretKey.fromPem(pemFileContent));

let jsonContent: string = readFileSync("../output/tokenstaking.abi.json", { encoding: "utf8" });
let json = JSON.parse(jsonContent);
let abiRegistry = AbiRegistry.create(json);

const ProposalCreationArgsType: StructType = abiRegistry.getStruct("ProposalCreationArgs");
const ActionType: StructType = abiRegistry.getStruct("Action");
const VoteType: EnumType = abiRegistry.getEnum("VoteType");
const StatusType: EnumType = abiRegistry.getEnum("ProposalStatus");

let abi = new SmartContractAbi(abiRegistry, ["Governance"]);

async function deployContract() {
  
    let alice = new Account(signer.getAddress());
    let aliceOnNetwork = await networkProvider.getAccount(signer.getAddress());
    alice.update(aliceOnNetwork);
    
    let buffer: Buffer = await promises.readFile("../output/tokenstaking.wasm");
    let code = Code.fromBuffer(buffer);
    let contract_address = readFileSync("config.txt", { encoding: "utf8" }) 
    let contract = new SmartContract({ address: new Address(contract_address), abi: abi});

    let tx: Transaction = contract.upgrade({
        code: code,
        codeMetadata: new CodeMetadata(true, true, true, true),
        initArguments: [
           new BigUIntValue(1),
           new U64Value(1),
           new U64Value(20),
           new BigUIntValue(1),
           new AddressValue(new Address("erd19gynnulh8qy3t27tu5ce02rhxv5ec74crdc0gh5x54lryc9jpeaqhsxat6")),
           new AddressValue(new Address("erd19gynnulh8qy3t27tu5ce02rhxv5ec74crdc0gh5x54lryc9jpeaqhsxat6")),
           new U64Value(60 * 2),
           new U64Value(60 * 2),
        ],
        gasLimit: 50000000,
        chainID: "D"
    })

    tx.setNonce(alice.getNonceThenIncrement());
    await signer.sign(tx);
    let txHash = await networkProvider.sendTransaction(tx);

    let contractAddress = SmartContract.computeAddress(tx.getSender(), tx.getNonce());
    // writeFileSync("config.txt", contractAddress.bech32(), { encoding: "utf8" });
    // console.log("contract address: ", contractAddress.bech32());
    return contractAddress.bech32();
}

async function createProposal(contract, description) {
    let alice = new Account(signer.getAddress());
    let aliceOnNetwork = await networkProvider.getAccount(signer.getAddress());
    alice.update(aliceOnNetwork);

    let target = new Address("erd1qqqqqqqqqqqqqpgq4p0xgyt0smpgxt8czurwp26z3rskr8hmpeaqevc39r");
    let propsalCreationArgs: Struct = new Struct(
        ProposalCreationArgsType, 
        [
            new Field(BytesValue.fromUTF8(description), "description"),
            new Field(new List(new ListType(ActionType), [
                new Struct(
                    ActionType,
                    [
                        new Field(new U64Value(3000000), "gas_limit"),
                        new Field(new AddressValue(target), "dest_address"),
                        new Field(new List(new ListType(new BytesType()), []), "payments"),
                        new Field(BytesValue.fromUTF8("changeQuorum"), "endpoint_name"),
                        new Field(new List(new ListType(new BytesType()), [
                            BytesValue.fromHex("03e8")                         
                        ]), "arguments"),
                    ]
                ),
            ]), "actions")
        ]
    )

    let tx = contract.methodsExplicit.propose([   
        propsalCreationArgs
    ])
        .withNonce(alice.getNonceThenIncrement())
        .withGasLimit(5000000)
        .withChainID('D')
        .buildTransaction();

    await signer.sign(tx);
    await networkProvider.sendTransaction(tx);
}

async function voteProposal(contract, proposalId, voting) {
    let alice = new Account(signer.getAddress());
    let aliceOnNetwork = await networkProvider.getAccount(signer.getAddress());
    alice.update(aliceOnNetwork);

    let tx;
    if(voting == 0) {
        tx = contract.methodsExplicit.upvote([   
            new U64Value(proposalId),
        ])
            .withNonce(alice.getNonceThenIncrement())
            .withGasLimit(4000000)
            .withChainID('D')
            .buildTransaction();
    } else {
        tx = contract.methodsExplicit.downvote([   
            new U64Value(proposalId),
        ])
            .withNonce(alice.getNonceThenIncrement())
            .withGasLimit(4000000)
            .withChainID('D')
            .buildTransaction();
    }

    await signer.sign(tx);
    await networkProvider.sendTransaction(tx);
    
    // await new TransactionWatcher(networkProvider).awaitCompleted(tx);
}

async function queueProposal(contract, proposalId) {
    let alice = new Account(signer.getAddress());
    let aliceOnNetwork = await networkProvider.getAccount(signer.getAddress());
    alice.update(aliceOnNetwork);

    
    let tx = contract.methodsExplicit.queue([   
        new U64Value(proposalId),
    ])
        .withNonce(alice.getNonceThenIncrement())
        .withGasLimit(3000000)
        .withChainID('D')
        .buildTransaction();

    await signer.sign(tx);
    await networkProvider.sendTransaction(tx);
}

async function cancelProposal(contract, proposalId) {
    let alice = new Account(signer.getAddress());
    let aliceOnNetwork = await networkProvider.getAccount(signer.getAddress());
    alice.update(aliceOnNetwork);

    
    let tx = contract.methodsExplicit.cancel([   
        new U64Value(proposalId),
    ])
        .withNonce(alice.getNonceThenIncrement())
        .withGasLimit(3000000)
        .withChainID('D')
        .buildTransaction();

    await signer.sign(tx);
    await networkProvider.sendTransaction(tx);
}

async function executeProposal(contract, proposalId) {
    let alice = new Account(signer.getAddress());
    let aliceOnNetwork = await networkProvider.getAccount(signer.getAddress());
    alice.update(aliceOnNetwork);

    let tx = contract.methodsExplicit.execute([   
        new U64Value(proposalId),
    ])
        .withNonce(alice.getNonceThenIncrement())
        .withGasLimit(8000000)
        .withChainID('D')
        .buildTransaction();

    await signer.sign(tx);
    await networkProvider.sendTransaction(tx);
}

async function getProposal(contract, proposalId) {
    let resultsParser = new ResultsParser();

    let interaction = <Interaction>contract.methods.getProposal([proposalId]);
    let query = interaction.check().buildQuery();
    let queryResponse =  await networkProvider.queryContract(query);
    let endpointDefinition = interaction.getEndpoint();
    let { firstValue, returnCode } = resultsParser.parseQueryResponse(queryResponse, endpointDefinition);
    let value = <Struct>firstValue;
    console.log("Upvotes: ", Number(value.getFieldValue("num_upvotes")));
    console.log("Downvotes: ", Number(value.getFieldValue("num_downvotes")));
    console.log("Eta: ", Number(value.getFieldValue("eta")));
    console.log("Description: ", value.getFieldValue("description").toString());
    console.log("Proposal id: ", Number(value.getFieldValue("id")))
    console.log("Was canceled: ", value.getFieldValue("was_canceled"))
    console.log("Was executed: ", value.getFieldValue("was_executed"))
    console.log("actions", value.getFieldValue("actions"));
}

async function getQuorum(contract) {
    let resultsParser = new ResultsParser();

    let interaction = <Interaction>contract.methods.getQuorum([]);
    let query = interaction.check().buildQuery();
    let queryResponse =  await networkProvider.queryContract(query);
    let endpointDefinition = interaction.getEndpoint();
    let { firstValue, returnCode } = resultsParser.parseQueryResponse(queryResponse, endpointDefinition);
    let quorum = <BigUIntValue> firstValue;
    console.log(quorum.value.toString());
}

async function getProposalStatus(contract, proposalId) {
    let resultsParser = new ResultsParser();

    let interaction = <Interaction>contract.methods.state([proposalId]);
    let query = interaction.check().buildQuery();
    let queryResponse =  await networkProvider.queryContract(query);
    let endpointDefinition = interaction.getEndpoint();
    let { firstValue, returnCode } = resultsParser.parseQueryResponse(queryResponse, endpointDefinition);
    let value = <EnumValue>firstValue;
    console.log("Status: ", value.valueOf().name);
    // console.log("Upvotes: ", Number(value.getFieldValue("num_upvotes")));
    // console.log("Downvotes: ", Number(value.getFieldValue("num_downvotes")));
}

async function runTest() {
    let contract_address = readFileSync("config.txt", { encoding: "utf8" }) 
    // console.log("contract address: ", contract_address);
   
    let contract = new SmartContract({ address: new Address(contract_address), abi: abi });
    // let interaction = <Interaction>contract.methods.getVotingPeriodInBlocks();
    // let query = interaction.check().buildQuery();
    // let queryResponse =  await networkProvider.queryContract(query);
    // let endpointDefinition = interaction.getEndpoint();
    // let { firstValue, returnCode } = resultsParser.parseQueryResponse(queryResponse, endpointDefinition);
    // let value = <U64Value>firstValue;
    // console.log(value);

    // await createProposal(contract, "new Proposal");
    //await createProposal(contract, "proposal2");

    // await voteProposal(contract, 4, 1);   //Downvote
//    await voteProposal(contract, 14, 0); // Upvote

    //  await getProposal(contract, 15);
    // await cancelProposal(contract, 8);

    //  await getProposalStatus(contract, 14);

//    await queueProposal(contract, 18);
    //  await executeProposal(contract, 18);
    // await getQuorum(contract);
}

(async() => {
    // await deployContract();
    await runTest();
})()



