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
  }= require('@elrondnetwork/erdjs');
const { Signature } = require('@elrondnetwork/erdjs/out/signature');

let apiUrl = "https://testnet-gateway.elrond.com";
let provider = new ProxyProvider(apiUrl, { timeout: 6000 });

NetworkConfig.getDefault().sync(provider);

let pemFileContent = readFileSync("wallet-owner.pem", { encoding: "utf8" }).trim();
let signer = new UserSigner(UserSecretKey.fromPem(pemFileContent));


const getScWasmCode = (filePath) => {
    const rawFile = readFileSync(filePath);
    const fileString = rawFile.toString('utf8');
    return Code.fromBuffer(Buffer.from(fileString, 'hex'));
}

const deployedContract = async (contractaddr, token_id) => {
    let alice = new Account(signer.getAddress());
    await alice.sync(provider);
    const code = await getScWasmCode("wasmcode-game.txt");
    
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
           new TokenIdentifierValue(Buffer.from(token_id))
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

const gameTest = async () => {
    const contract_addr = "erd1qqqqqqqqqqqqqpgqr0p0jv3r7gujp0ct4v6us5ea3f8wklm3peaq080yh2";
    let response, decoded;
    //// contract deploy on testnet
    let contract;
    if(contract_addr == "") contract = await deployedContract();
    else contract = await deployedContract(contract_addr, "GAND-123456");
    console.log('deployed contract address: "' + contract.getAddress().toString() + '"');

    //// Start the game by owner ---------- backend
    await runTransaction(contract, {
        func: new ContractFunction("startGame"),
        gasLimit: new GasLimit(5000000),
        args: [],
    });

    //// Get the game start time ---------- backend & frontend
    response = await contract.runQuery(provider, {
        func: new ContractFunction("getGameStartTime"),
        args: []
    });
    decoded = Buffer.from(response.returnData[0], "base64").toString("hex");
    console.log("Game start time : ", decoded);

    //// Get the time reward ---------- backend & frontend
    response = await contract.runQuery(provider, {
        func: new ContractFunction("getTimeReward"),
        args: []
    });
    decoded = Buffer.from(response.returnData[0], "base64").toString("hex");
    console.log("Time reward : ", decoded);

    //// Set the NFT whitelist by owner ---------- backend
    const setRandomNFTs = ["RANDOM-4567be-5", "STICKY-123456-2", "SKINR-b23374-1", "SKINR-b23374-2"];
    const setPartnerNFTs = ["PAT-4567eb-1", "STOCK-283739-3"];
    const setNativeNFTs = ["NAT-976382-1", "NFT56-57282e-1", "NFTFE-839244e-2"];

    let setNfts = [...setRandomNFTs, ...setPartnerNFTs, ...setNativeNFTs];
    let setNftList = new List(
        new ListType(new TokenIdentifierType()), 
        setNfts.map((tokenName) => new TokenIdentifierValue(Buffer.from(tokenName)))
    );

    await runTransaction(contract, {
        func: new ContractFunction("setWhitelist"),
        gasLimit: new GasLimit(5000000),
        args: [ setNftList, new U8Value(setRandomNFTs.length), new U8Value(setPartnerNFTs.length) ],
    });

    //// Update the NFT whitelist by owner ---------- backend
    const updateRandomNFTs = ["RANDOM-4567be-4", "STICKY-123456-3"];
    const updatePartnerNFTs = [];
    const updateNativeNFTs = ["STIKE-34582-1", "BOLD-13243e-1"];

    let updateNfts = [...updateRandomNFTs, ...updatePartnerNFTs, ...updateNativeNFTs];
    let updateNftList = new List(
        new ListType(new TokenIdentifierType()), 
        updateNfts.map((tokenName) => new TokenIdentifierValue(Buffer.from(tokenName)))
    );
    await runTransaction(contract, {
        func: new ContractFunction("updateWhitelist"),
        gasLimit: new GasLimit(5000000),
        args: [ updateNftList, new U8Value(updateRandomNFTs.length), new U8Value(updatePartnerNFTs.length) ],
    });

    //// Get the multiplier for player ---------- backend
    let player = new Address("erd19gynnulh8qy3t27tu5ce02rhxv5ec74crdc0gh5x54lryc9jpeaqhsxat6");
    let skinNFT = Buffer.from("NFT56-57282e-1");
    response = await contract.runQuery(provider, {
        func: new ContractFunction("getMultiplier"),
        args: [new AddressValue(player), new TokenIdentifierValue(skinNFT)]
    });
    decoded = Buffer.from(response.returnData[0], "base64").toString("hex");
    console.log("Time reward for player: ", decoded);

    //// Sign the player's reward amount by owner ---------- backend
    /*  Please check the testgame_signature.js  */

    //// Claim the reward ---------- frontend
    let rewardAmount = 20000;
    let signature = Buffer.from("30440220513ac76631c265770865048ad23230a3df13eece42a291dd8dea6a3e31fa327402206bf414f51d9f91fb0b68b35cf6bbaada2ad76bfeb88d7460cacda158354b33b7", "hex")
 
    await runTransaction(contract, {
        func: new ContractFunction("claimReward"),
        gasLimit: new GasLimit(8000000),
        args: [ new BigUIntValue(new BigNumber(rewardAmount)), new BytesValue(signature) ],
    });

    let player1 = new Address("erd19gynnulh8qy3t27tu5ce02rhxv5ec74crdc0gh5x54lryc9jpeaqhsxat6");
    response = await contract.runQuery(provider, {
        func: new ContractFunction("getClaimCount"),
        args: [new AddressValue(player1)]
    });
    console.log(response);
    decoded = Buffer.from(response.returnData[0], "base64").toString("hex");
    console.log("Time reward for player: ", decoded);
}

(async() => {
    await gameTest();
})();
