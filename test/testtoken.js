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
    decodeBigNumber,
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
    const code = await getScWasmCode("wasmcode-token.txt");
    
    if(contractaddr !== undefined) {
        return new SmartContract({
            address: new Address(contractaddr)
        });
    }

    const contract = new SmartContract({ address: contractaddr });

    let tx = contract.deploy({
        code,
        codeMetadata: new CodeMetadata(true, true, false),
        gasLimit: new GasLimit(50000000),
        initArguments: [
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

const tokenTest = async () => {
    const contract_addr = "";
    let response, decoded;

    //// token contract deploy on testnet
    let tokencontract;
    if(contract_addr == "") tokencontract = await deployedContract();
    else tokencontract = await deployedContract(contract_addr);
    console.log(`deployed contract address: ${tokencontract.getAddress().toString()}`);

    //// get the token total supply 
    response = await tokencontract.runQuery(provider, {
        func: new ContractFunction("getTotalSupply"),
        args: []
    });
    decoded = Buffer.from(response.returnData[0], "base64").toString("hex");
    let totalSupply = new BigNumber(decoded, 16);
    console.log("Total Supply: ", totalSupply.toString());

    //// get the status of token distribute 
    response = await tokencontract.runQuery(provider, {
        func: new ContractFunction("getTokenDistributeEnded"),
        args: []
    });
    decoded = Buffer.from(response.returnData[0], "base64").toString();
    console.log("Token distributed: ", (decoded == "")?"false": "true");

    //issue tokens
    await runTransaction(tokencontract, {
        func: new ContractFunction("issueTokens"),
        gasLimit: new GasLimit(100000000),
        value:Balance.egld(0.05),
        args: [
            new BytesValue(Buffer.from("GANDTICK")),
            new BytesValue(Buffer.from("GAND")),

        ],
    });
    await delay(25000);

    //// get the issued token id 
    response = await tokencontract.runQuery(provider, {
        func: new ContractFunction("getIssueTokenId"),
        args: []
    });


    decoded = Buffer.from(response.returnData[0], "base64").toString();
    console.log("Issued Token Id: ", decoded);

    //// token distribute
    let game_address = new AddressValue(new Address("erd19gynnulh8qy3t27tu5ce02rhxv5ec74crdc0gh5x54lryc9jpeaqhsxat6"));
    let dao_address = new AddressValue(new Address("erd1pw3zsq9u0kcej69hnrkrq009zny4yxvkpg7zz4tdrmvf4j24gzlsltw3h4"));
    let liquidity_address = new AddressValue(new Address("erd1pw3zsq9u0kcej69hnrkrq009zny4yxvkpg7zz4tdrmvf4j24gzlsltw3h4"));
    let staking_address = new AddressValue(new Address("erd1pw3zsq9u0kcej69hnrkrq009zny4yxvkpg7zz4tdrmvf4j24gzlsltw3h4"));
    let marketing_address = new AddressValue(new Address("erd1pw3zsq9u0kcej69hnrkrq009zny4yxvkpg7zz4tdrmvf4j24gzlsltw3h4"));
    let team_address = new AddressValue(new Address("erd1pw3zsq9u0kcej69hnrkrq009zny4yxvkpg7zz4tdrmvf4j24gzlsltw3h4"));

    await runTransaction(tokencontract, {
        func: new ContractFunction("distributeToken"),
        gasLimit: new GasLimit(10000000),
        args: [
            game_address,
            dao_address,
            liquidity_address,
            staking_address,
            marketing_address,
            team_address
        ],
    });

    //// get the balance of  the address
    let address = game_address;
    response = await tokencontract.runQuery(provider, {
        func: new ContractFunction("getBalanceOf"),
        args: [address]
    });


    decoded = Buffer.from(response.returnData[0], "base64").toString("hex");
    let amount = new BigNumber(decoded, 16);
    console.log(`Balance of ${address}: ${amount}`);
}

(async() => {
    await tokenTest();
})();

