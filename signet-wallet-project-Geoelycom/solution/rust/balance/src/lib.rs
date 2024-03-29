extern crate bitcoincore_rpc;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use bs58;
use hex;
use hmac_sha512::HMAC;
use num_bigint::BigUint; // for modulus math on large numbers
use ripemd::Ripemd160;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};
use std::error::Error;
use serde::Serialize;
use serde_json;
use std::io::Read;
use std::path::PathBuf;
use std::str;

// Provided by administrator
const WALLET_NAME: &str = "wallet_000";

// xprv
const EXTENDED_PRIVATE_KEY: &str = tprv8ZgxMBicQKsPfCxvMSGLjZegGFnZn9VZfVdsnEbuzTGdS9aZjvaYpyh7NsxsrAc8LsRQZ2EYaCfkvwNpas8cKUBbptDzadY7c3hUi8i33XJ;

const HARDENED_OFFSET: u32 = 2_u32.pow(31);

#[derive(Serialize)]
struct ExtendedKey {
    // create a struct that will hold the data of our extended keys
    version: [u8; 4],            // 4 bytes
    depth: u8,                   // 1 byte
    parent_fingerprint: [u8; 4], // 4 bytes
    child_number: [u8; 4],       // 4 bytes
    chain_code: [u8; 32],        // 32 bytes
    key_data: Vec<u8>,           // 33 bytes
}

#[derive(Serialize)]
struct ChildKey {
    pub private_key: Vec<u8>,
    pub chain_code: Vec<u8>,
}

pub struct OutgoingTx {}

struct SpendingTx {}

// final wallet state struct
pub struct WalletState {
    utxos: Vec<OutgoingTx>,
    witness_programs: Vec<[u8; 22]>,
    public_keys: Vec<[u8; 33]>,
    private_keys: Vec<[u8; 32]>,
}

// Decode a base58 string into an array of bytes
fn base58_decode(base58_string: &str) -> Result<Vec<u8>, String> {
    let base58_alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    // Convert Base58 string to a big integer
    let mut biguint = BigUint::from(0u32);
    for character in base58_string.chars() {
        let value = match base58_alphabet.find(character) {
            Some(val) => BigUint::from(val as u32),
            None => return Err("Invalid base58 string character".to_string()), // Return an empty vector on invalid character
        };
        biguint = biguint * 58u32 + value;
    }
    // Convert the integer to bytes
    let bytes = biguint.to_bytes_be();
    // Chop off the 32 checksum bits and return
    //check if the byte array is long enough to include the checksum
    if bytes.len() < 4 {
        return Err("invalid input: too short for a checksum".into());
    }
    //Truncate the last 4 bytes = (seperate the payload and the checksum)
    let checksum_index = bytes.len() - 4;
    // BONUS POINTS: Verify the checksum!
    let (data, original_checksum) = bytes.split_at(checksum_index);
    //compute the checksum of the data
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash_once = hasher.finalize();
    let mut hasher = Sha256::new();
    hasher.update(&hash_once);
    let hash_twice = hasher.finalize();
    let computed_checksum = hash_twice[0..4].to_vec();
    // //compare the computed checksum with the provided checksum
    if computed_checksum != original_checksum {
        return Err("checksum verification failed".into());
    }
    Ok(data.to_vec())
}

// Deserialize the extended key bytes and return a JSON object
// https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#serialization-format
// 4 byte: version bytes (mainnet: 0x0488B21E public, 0x0488ADE4 private; testnet: 0x043587CF public, 0x04358394 private)
// 1 byte: depth: 0x00 for master nodes, 0x01 for level-1 derived keys, ....
// 4 bytes: the fingerprint of the parent's key (0x00000000 if master key)
// 4 bytes: child number. This is ser32(i) for i in xi = xpar/i, with xi the key being serialized. (0x00000000 if master key)
// 32 bytes: the chain code
// 33 bytes: the public key or private key data (serP(K) for public keys, 0x00 || ser256(k) for private keys)
fn deserialize_key(bytes: Vec<u8>) -> Result<(String, ExtendedKey), String> {
    if bytes.len() != 78 {
        return Err("Invalid byte length of an extended key".to_string());
    }
    let version = bytes[0..4]
        .try_into()
        .map_err(|_| "failed to extract version".to_string())?;
    let depth = bytes[4];
    let parent_fingerprint = bytes[5..9]
        .try_into()
        .map_err(|_| "failed to extract parent fingerprint".to_string())?;
    let child_number = bytes[9..13]
        .try_into()
        .map_err(|_| "failed to extract child number".to_string())?;
    let chain_code = bytes[13..45]
        .try_into()
        .map_err(|_| "failed to extract chain_code".to_string())?;
    let key_data = bytes[45..78].to_vec();

    let ext_key = ExtendedKey {
        version,
        depth,
        parent_fingerprint,
        child_number,
        chain_code,
        key_data,
    };

    let json_string = serde_json::to_string(&ext_key).map_err(|e| e.to_string())?;

    Ok((json_string, ext_key))
}

// Derive the secp256k1 compressed public key from a given private key
// BONUS POINTS: Implement ECDSA yourself and multiply your key by the generator point!
fn derive_public_key_from_private(key_data: &[u8]) -> Result<[u8; 33], String> {
    //create a Secp256k1 context object
    let secp = Secp256k1::new();
    if key_data.len() < 33 {
        return Err("invalid key data length".to_string());
    }
    //Extract the first 32 bytes for the secret key after skipping the first byte
    let secret_key = SecretKey::from_slice(&key_data[1..]) //skip the first byte
        .map_err(|e| e.to_string())?;
    // Derive the public key
    // Implement ECDSA yourself and multiply your key by the generator point!
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    //serailize the key to a [u8; 33] array
    let serailized_public_key = public_key.serialize();

    Ok(serailized_public_key)
}

// Perform a BIP32 parent private key -> child private key operation
// Return a JSON object with "key" and "chaincode" properties as bytes
// https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#user-content-Private_parent_key_rarr_private_child_key
fn derive_priv_child(key: &[u8; 32], chaincode: &[u8; 32], index: u32) -> Result<String, String> {
    //check if the index is for hardened key
    if index < 0x80000000 {
        return Err("Index must be for a hardened key".to_string());
    }
    // serailize the index =  this is because the index needs to be converted into a 4-byte array. this we can use to_be_bytes method. this will ensure bytes are in big endain
    let index_bytes = index.to_be_bytes();
    //prep the data for hashing by concatenating the key and index bytes
    // prepend 0x00 to the parent private key for hardened key
    let mut data_to_hash = Vec::with_capacity(1 + key.len() + index_bytes.len());
    data_to_hash.push(0x00); //prepend 0x00 to the parent private key
    data_to_hash.extend_from_slice(key); // key (32 bytes)
    data_to_hash.extend_from_slice(&index_bytes); // index (4 bytes)
                                                  // we would use hmac-sha512 to hash the data. using the chaincode as the key for hmac operation
    let hmac_key = HMAC::mac(&data_to_hash, chaincode);
    // split the hmac_key into two halfs of 32 bytes
    let (left_half, right_half) = hmac_key.split_at(32);
    //convert the curve order of secp256k1 into a BigUint
    let curve_order_str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";
    let curve_order_bytes = curve_order_str.as_bytes();
    let curve_order =
        BigUint::parse_bytes(curve_order_bytes, 16).expect("failed to parse curve order");
    // convert the parent private key and the left half of the hmac result to Biguint for arithematic operations
    let parent_key_bigint = BigUint::from_bytes_be(key);
    let left_half_key_bigint = BigUint::from_bytes_be(left_half);
    // calculate = Add them together and take the modulo with the curve. the result is the child private key
    let child_private_key = (parent_key_bigint + left_half_key_bigint) % &curve_order;

    let child_key = ChildKey {
        private_key: child_private_key.to_bytes_be(),
        chain_code: right_half.to_vec(),
    };

    serde_json::to_string(&child_key).map_err(|e| e.to_string())
}

// Given an extended private key and a BIP32 derivation path, compute the child private key found at the last path
// The derivation path is formatted as an array of (index: int, hardened: bool) tuples.
// (weekend task)
fn get_child_key_at_path(key: [u8; 32], chaincode: [u8; 32], paths: Vec<(u32, bool)>) -> ChildKey {
  
}

// Compute the first N child private keys.
// Return an array of keys encoded as bytes.
fn get_keys_at_child_key_path(child_key: ChildKey, num_keys: u32) -> Vec<[u8; 32]> {}

// Derive the p2wpkh witness program (aka scriptPubKey) for a given compressed public key.
// Return a bytes array to be compared with the JSON output of Bitcoin Core RPC getblock
// so we can find our received transactions in blocks.
// These are segwit version 0 pay-to-public-key-hash witness programs.
// https://github.com/bitcoin/bips/blob/master/bip-0141.mediawiki#user-content-P2WPKH
fn get_p2wpkh_program(pubkey: [u8; 33]) -> [u8; 22] {}

// public function that will be called by `run` here as well as the spend program externally
pub fn recover_wallet_state(
    extended_private_key: &str,
    cookie_filepath: &str,
) -> Result<WalletState, Box<dyn Error>> {
    // Deserialize the provided extended private key

    // Derive the key and chaincode at the path in the descriptor (`84h/1h/0h/0`)

    // Get the child key at the derivation path

    // Compute 2000 private keys from the child key path
    // For each private key, collect compressed public keys and witness programs
    let private_keys = vec![];
    let public_keys = vec![];
    let witness_programs = vec![];

    // Collect outgoing and spending txs from a block scan
    let mut outgoing_txs: Vec<OutgoingTx> = vec![];
    let mut spending_txs: Vec<SpendingTx> = vec![];
    let mut utxos: Vec<OutgoingTx> = vec![];

    // set up bitcoin-core-rpc on signet
    let path = PathBuf::from(cookie_filepath);
    let rpc = Client::new("http://localhost:38332", Auth::CookieFile(path))?;

    // Scan blocks 0 to 300 for transactions
    // Check every tx input (witness) for our own compressed public keys. These are coins we have spent.
    // Check every tx output for our own witness programs. These are coins we have received.
    // Keep track of outputs by their outpoint so we can check if it was spent later by an input
    // Collect outputs that have not been spent into a utxo set
    // Return Wallet State
    Ok(WalletState {
        utxos,
        public_keys,
        private_keys,
        witness_programs,
    })
}

pub fn run(rpc_cookie_filepath: &str) -> Result<(), ()> {
    let utxos = recover_wallet_state(EXTENDED_PRIVATE_KEY, rpc_cookie_filepath)?;
    let balance: println!("{} {:.8}", WALLET_NAME, balance);
    Ok(())
}
