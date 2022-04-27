let elliptic = require('elliptic');
const sha3 = require('js-sha3');
const ec = new elliptic.ec('secp256k1');
const keccak256 = require('keccak256')
const sha256 = require('js-sha256');
const BigNumber = require('bignumber.js');
const { Address, AddressValue } = require('@elrondnetwork/erdjs');

const signByOwner = (player, reward_amount, privKey, claimcount) => {
    let address = new Address(player);
    let reward_num = new BigNumber(reward_amount);
    let reward_hex = reward_num.toString(16);
    if(reward_hex.length % 2 == 1) reward_hex = "0" + reward_hex;

    let msg = reward_hex + address.valueHex + claimcount;
    msgHash = keccak256(Buffer.from(msg, 'hex'))

   // console.log(msgHash.toString('hex'));
    const sha256Hasher = sha256.create();
    sha256Hasher.update(msgHash);
    msgHash = sha256Hasher.hex();
   // console.log(msgHash, privKey);
    
    let signature = ec.sign(msgHash, privKey, "hex", {canonical: true});

    let strR = signature.r.toString(16);
    let strS = signature.s.toString(16);
   // console.log("Signature:", strR, strR.length, strS, strS.length);

    let lr = signature.r.byteLength();
    if((signature.r.toBuffer()[0] & 0x80) != 0) lr++, strR = "00" + strR;
    let ls = signature.s.byteLength();
    if((signature.s.toBuffer()[0] & 0x80) != 0) ls++, strS = "00" + strS;

    let tot = lr + ls + 4;
    let signedMsg = '30' + tot.toString(16) + '02' + lr.toString(16) + strR + '02' + ls.toString(16) + strS;
    return signedMsg;
}

const generateKeyPair = () => {
    let keyPair = ec.keyFromPrivate("97ddae0f3a25b92268175400149d65d6887b9cefaf28ea2c078e05cdc15a3c0a");
    let pubKey = keyPair.getPublic(true, "hex");
    let privKey = keyPair.getPrivate("hex");

    return {
        pubKey, privKey
    }   
}

const { pubKey, privKey } = generateKeyPair();
console.log("public key: ",  pubKey, ' ', pubKey.length); // Please set the PUBLIC_KEY of game contract with pubKey
console.log("private key: ", privKey, ' ', privKey.length); // Please use as the privKey of owner on backend side 

let signedMsg = signByOwner("erd19gynnulh8qy3t27tu5ce02rhxv5ec74crdc0gh5x54lryc9jpeaqhsxat6", 20000, privKey, "00000001");
console.log(signedMsg);
