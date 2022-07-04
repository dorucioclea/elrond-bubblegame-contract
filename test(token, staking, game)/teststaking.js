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
const { decode } = require('punycode');

// let apiUrl = "https://testnet-gateway.elrond.com";
let apiUrl = "https://devnet-gateway.elrond.com";
let provider = new ProxyProvider(apiUrl, { timeout: 50000 });

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

const upgradeContract = async (contractaddr) => {

    let alice = new Account(signer.getAddress());
    await alice.sync(provider);
    const code = await getScWasmCode("wasmcode-staking.txt");
    
    const contract = new SmartContract({ address: contractaddr });
    let tx = contract.upgrade({
        code: code,
        codeMetadata: new CodeMetadata(true, true, true, true),
        initArguments: [
            new TokenIdentifierValue(Buffer.from("GAND-ddd565")),
            new BigUIntValue(new BigNumber(0))
        ],
        gasLimit: new GasLimit(100000000),
    })
    
    tx.setNonce(alice.nonce);
    await signer.sign(tx);
   
    await tx.send(provider);
    await tx.awaitExecuted(provider);
    const txHash = tx.getHash();
    return contract;
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
        gasLimit: new GasLimit(100000000),
        initArguments: [
            new TokenIdentifierValue(Buffer.from("GAND-ddd565")),
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
    const contract_addr = "erd1qqqqqqqqqqqqqpgq4xcphujjh6khcwtj09dyxy37vu02p5tqpeaqt3aadx";
    let response, decoded;

    //// staking contract deploy on testnet
    if(contract_addr == "") stakingcontract = await deployedContract();
    else stakingcontract = await deployedContract(contract_addr);
    console.log(`deployed contract address: ${stakingcontract.getAddress().toString()}`);

    // //start staking
    // await runTransaction(stakingcontract, {
    //     func: new ContractFunction("startStaking"),
    //     gasLimit: new GasLimit(5000000),
    //     args:[]
    // });

    //// Get the staking  status
    response = await stakingcontract.runQuery(provider, {
        func: new ContractFunction("getStakingStatus"),
        args: []
    });

    decoded = Buffer.from(response.returnData[0], "base64").toString();
    console.log("staking status: ", (decoded == "")?false:true);

    //// stake token
    let token = new Token({identifier:"GAND-ddd565"});
    
    let interaction = new Interaction(stakingcontract, new ContractFunction("stake"), undefined, [
        new U32Value(0)
    ]);
    interaction = interaction.withSingleESDTTransfer(new Balance(token, 0, "100000000000000000000")).withGasLimit(60000000);
    let tx = interaction.buildTransaction();

    let alice = new Account(signer.getAddress());
    await alice.sync(provider);

    tx.setNonce(alice.nonce);
    await signer.sign(tx);
    await tx.send(provider);
    await tx.awaitExecuted(provider);

   
    //// Get the staking  info
    let player = new Address("erd19gynnulh8qy3t27tu5ce02rhxv5ec74crdc0gh5x54lryc9jpeaqhsxat6");
    response = await stakingcontract.runQuery(provider, {
        func: new ContractFunction("getStakingInfo"),
        args: [new AddressValue(player)]
    });

    response.assertSuccess();
    decoded = Buffer.from(response.returnData[0], "base64").toString("hex");

    const getStakingInfo = (hexString) => {
        const address = Address.fromHex(hexString.substring(0, 64)).bech32();
        hexString = hexString.substring(64);
        const len = Number.parseInt(hexString.substring(0, 8), 16);
        hexString = hexString.substring(8);
        const staked_amount = new BigNumber(hexString.substring(0, len * 2), 16);
        hexString = hexString.substring(len * 2);
        const stake_option = Number.parseInt(hexString.substring(0, 8), 16);
        hexString = hexString.substring(8);
        const lock_time = new BigNumber(hexString.substring(0, 16), 16);
        hexString = hexString.substring(16);
        const unlock_time = new BigNumber(hexString.substring(0, 16), 16);
        hexString = hexString.substring(16);
        const last_claim_time = new BigNumber(hexString.substring(0, 16), 16);
        hexString = hexString.substring(16);
        const from_day = Number.parseInt(hexString.substring(0, 8), 16);
        hexString = hexString.substring(8);
        const to_day = Number.parseInt(hexString.substring(0, 8), 16);
        hexString = hexString.substring(8);
        const last_claim_day = Number.parseInt(hexString.substring(0, 8), 16);
        hexString = hexString.substring(8);
        return {
            address, staked_amount, stake_option, lock_time, unlock_time, last_claim_time,
            from_day, to_day, last_claim_day
        }
    }
    const {address, staked_amount, stake_option, lock_time, unlock_time, last_claim_time, from_day, to_day, last_claim_day} = getStakingInfo(decoded);
    console.log(address, staked_amount, stake_option, lock_time, unlock_time, last_claim_time, from_day, to_day, last_claim_day);

   
    //// calc APR
    response = await stakingcontract.runQuery(provider, {
        func: new ContractFunction("calcAPR"),
        args: []
    });
    console.log(11111111111111, response.returnData);
    decoded = Buffer.from(response.returnData[0], "base64").toString("hex");
    let apr = new BigNumber(decoded, 16);
    console.log(`APR: ${apr}`);
    console.log(err);

    //// unstake 
    await runTransaction(stakingcontract, {
        func: new ContractFunction("unstake"),
        gasLimit: new GasLimit(5000000),
        args:[
        ]
    });

    //// get rewards
    response = await stakingcontract.runQuery(provider, {
        func: new ContractFunction("getRewards"),
        args: [new AddressValue(player)]
    });
    decoded = Buffer.from(response.returnData[0], "base64").toString("hex");
    let rewards = new BigNumber(decoded, 16);
    console.log(`Rewards: ${rewards}`);
    

    //// claim rewards
    await runTransaction(stakingcontract, {
        func: new ContractFunction("claim"),
        gasLimit: new GasLimit(5000000),
        args:[
        ]
    });
}

(async() => {
    await stakingTest();
    // await upgradeContract("erd1qqqqqqqqqqqqqpgq4xcphujjh6khcwtj09dyxy37vu02p5tqpeaqt3aadx");
})();


