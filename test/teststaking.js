const { readFileSync, accessSync, constants } = require('fs');
const BigNumber = require('bignumber.js');
const {
    ProxyProvider,
    IProvider,
    NetworkConfig,
    SmartContract,
    Account,
    parseUserKey,
    UserSigner,
    SmartContractAbi,
    Code,
    GasLimit,
    AbiRegistry,
    Address,
    ContractFunction,
    BytesValue,
    Balance,
    U32Value,
    BigUIntValue,
    AddressValue,
    Transaction,
    TransactionPayload,
    QueryResponse,
    TypedValue,
    CodeMetadata,
    BooleanValue,
    List,
    ListType,
    AddressType,
    UserSecretKey,
    TokenIdentifierValue,
    U64Value,
    U8Value,
    U8Type,
    TokenIdentifierType,
    Token,
    decodeBigNumber,
    Interaction,
    Egld,
  }= require('@elrondnetwork/erdjs');
const { Signature } = require('@elrondnetwork/erdjs/out/signature');

let apiUrl = "https://testnet-gateway.elrond.com";
let provider = new ProxyProvider(apiUrl, { timeout: 6000 });

NetworkConfig.getDefault().sync(provider);

let pemFileContent = readFileSync("wallet-owner.pem", { encoding: "utf8" }).trim();
let signer = new UserSigner(UserSecretKey.fromPem(pemFileContent));


function delay(ms) {
    return new Promise((resolve) => {
      setTimeout(resolve, ms);
    });
}

const getScWasmCode = (filePath) => {
    const rawFile = readFileSync(filePath);
    const fileString = rawFile.toString('utf8');
    return Code.fromBuffer(Buffer.from(fileString, 'hex'));
}

const deployedContract = async (contractaddr) => {
    let alice = new Account(signer.getAddress());
    await alice.sync(provider);
    const code = await getScWasmCode("wasmcode-staking.txt");
    
    if(contractaddr !== undefined) {
        return new SmartContract({
            address: new Address(contractaddr)
        });
    }

    const contract = new SmartContract({ address: contractaddr });

    
    let tx = contract.deploy({
        code,
        codeMetadata: new CodeMetadata(true, true, true, true),
        gasLimit: new GasLimit(50000000),
        initArguments: [
            new TokenIdentifierValue(Buffer.from("GAND-d16156")),
            new BigUIntValue(new BigNumber(0))            
        ]
    });

    tx.setNonce(alice.nonce);
    await signer.sign(tx);
    await tx.send(provider);
    await tx.awaitExecuted(provider);
    const txHash = tx.getHash();
    return contract;
}

async function runTransaction(contract, callArg) {
    let alice = new Account(signer.getAddress());
    await alice.sync(provider);

    let tx = contract.call(callArg);
    tx.setNonce(alice.nonce);
    await signer.sign(tx);
    await tx.send(provider);
    await tx.awaitExecuted(provider);
}

let stakingcontract;


const stakingTest = async () => {
    const contract_addr = "erd1qqqqqqqqqqqqqpgqygfyzc8qwut2svxypmygveftxzv8tlurpeaqjx7un7";
    let response, decoded;

    //// staking contract deploy on testnet
    if(contract_addr == "") stakingcontract = await deployedContract();
    else stakingcontract = await deployedContract(contract_addr);
    console.log(`deployed contract address: ${stakingcontract.getAddress().toString()}`);

     //start staking
     await runTransaction(stakingcontract, {
        func: new ContractFunction("startStaking"),
        gasLimit: new GasLimit(5000000),
        args:[]
    });

    //// Get the staking  status
    response = await stakingcontract.runQuery(provider, {
        func: new ContractFunction("getStakingStatus"),
        args: []
    });

    decoded = Buffer.from(response.returnData[0], "base64").toString();
    console.log("staking status: ", (decoded == "")?false:true);

    //// stake token
    let token = new Token({identifier:"GAND-d16156"});
    
    let interaction = new Interaction(stakingcontract, new ContractFunction("stake"), undefined, [
        new U32Value(0)
    ]);
    interaction = interaction.withSingleESDTTransfer(new Balance(token, 0, "100000000000000000000")).withGasLimit(8000000);
    let tx = interaction.buildTransaction();

    let alice = new Account(signer.getAddress());
    await alice.sync(provider);

    tx.setNonce(alice.nonce);
    await signer.sign(tx);
    await tx.send(provider);
    await tx.awaitExecuted(provider);

    //// calc APR
    response = await stakingcontract.runQuery(provider, {
        func: new ContractFunction("calcAPR"),
        args: []
    });
    console.log(response);
    decoded = Buffer.from(response.returnData[0], "base64").toString("hex");
    let apr = new BigNumber(decoded, 16);
    console.log(`APR: ${apr}`);

    //// unstake 
    await runTransaction(stakingcontract, {
        func: new ContractFunction("unstake"),
        gasLimit: new GasLimit(5000000),
        args:[
            new U32Value(0)
        ]
    });
}

(async() => {
    await stakingTest();
})();

