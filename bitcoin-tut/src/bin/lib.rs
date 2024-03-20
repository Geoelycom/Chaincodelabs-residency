extern crate bitcoincore_rpc;
use bitcoincore_rpc::bitcoin::key;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use bs58;
use hex;
use hmac_sha512::HMAC;
use num_bigint::BigUint; // for modulus math on large numbers
use ripemd::Ripemd160;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use serde::Serialize;
use serde_json;
use sha2::{Digest, Sha256};
use std::error::Error;
use std::io::Read;
use std::path::PathBuf;
use std::str::{self, FromStr};

fn main() {
    // deserailize_key = takes 78 bytes array;
    //deriv_priv_child = takes three arguement;
    let extended_key_bytes: [u8; 78] = [
        4, 53, 131, 148, // version (4 bytes)
        0, // depth (1 byte)
        0, 0, 0, 0, // parent_fingerprint (4 bytes)
        0, 0, 0, 0, // child_number (4 bytes)
        234, 111, 99, 186, 187, 61, 197, 197, 142, 164, 205, 17, 203, 63, 201, 215, 186, 165, 28,
        14, 20, 190, 130, 48, 255, 184, 177, 105, 103, 150, 166, 63, // chain_code (32 bytes)
        0, 60, 206, 72, 200, 79, 34, 52, 60, 189, 172, 142, 127, 37, 46, 216, 202, 17, 252, 227, 41,
        222, 174, 126, 214, 53, 183, 56, 34, 223, 237, 156, 119, // key_data (33 bytes)
    ];

    let bytes = extended_key_bytes.to_vec();
    //deserialize the bytes vector
    let deserailize_result = deserialize_key(bytes);

    // create a variable to hold the key_data 32 bytes array=note, first byte 0 has been removed for testing purposes
    let key_data_value = [60, 206, 72, 200, 79, 34, 52, 60, 189, 172, 142, 127, 37, 46, 216, 202, 17, 252, 227, 41,
    222, 174, 126, 214, 53, 183, 56, 34, 223, 237, 156, 119];

    match deserailize_result {
        Ok((json_string, ext_key)) => {
            println!("Deserailized JSON: {}", json_string);
            // we already initialized key_data to be an exact 32 bytes above by chopping of the first byte. and we have access to use ext_key.chain_code in derive_priv child we have to check and confirm that chain_code is 32 bytes, if not we would get an error
            if  ext_key.chain_code.len() != 32 {
                println!("Invalid chain_code length");
                return;
            }

            // convert vec<u8> to [u8; 32] for chain_code
            let chain_code_array:[u8; 32] = ext_key.chain_code.try_into().expect("Invalid chain_code length");

            // define index for our child key derivation for hardnened keys. (use index >= 0x80000000)
            let index:u32 = 0x80000000; // Example index for a hardened key
            match derive_priv_child(&key_data_value, &chain_code_array, index) {
              Ok(child_key_json) => {
                println!("Derived child key JSON: {}", child_key_json);
              }
              Err(e) => {
                print!("Error deriving child key: {}", e)
              }
            }
        }

        Err(e) => {
          println!("Error deserializing key: {}", e);
        } 
    }
}

#[derive(Serialize)]
pub struct ExtendedKey {
    // create a struct that will hold the data of our extended key
    pub version: [u8; 4],            // 4 bytes
    pub depth: u8,                   // 1 byte
    pub parent_fingerprint: [u8; 4], // 4 bytes
    pub child_number: [u8; 4],       // 4 bytes
    pub chain_code: [u8; 32],        // 32 bytes
    pub key_data: Vec<u8>,           // 33 bytes
}

#[derive(Serialize)]
pub struct ChildKey {
    pub private_key: Vec<u8>,
    pub chain_code: Vec<u8>,
}

// Decode a base58 string into an array of bytes
#[allow(dead_code)]
fn base58_decode(base58_string: &str) -> Result<Vec<u8>, String> {
    let base58_alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    // Convert Base58 string to a big integer
    let mut biguint = BigUint::from(0u32);
    // map each base58 string to a value
    // interate over each character in base58 string
    // For each character, find its position (value) in the base58_alphabet. This position is its numeric value in Base58 encoding.
    for character in base58_string.chars() {
        let value = match base58_alphabet.find(character) {
            Some(val) => BigUint::from(val as u32),
            None => return Err("Invalid base58 string character".to_string()), // Return an empty vector on invalid character
        };
        biguint = biguint * 58u32 + value;
        // println!("{}", biguint);
    }
    // Convert the integer to bytes
    let bytes = biguint.to_bytes_be();
    // convert the bytes into an hex string for easy testing
    // let base58_hex_string:String = bytes.iter().map(|byte| format!("{:02x}", byte)).collect();
    // println!("hex string: {}", base58_hex_string);
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

#[allow(dead_code)]
pub fn deserialize_key(bytes: Vec<u8>) -> Result<(String, ExtendedKey), String> {
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
pub fn derive_public_key_from_private(key_data: &[u8]) -> Result<[u8; 33], String> {
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

#[allow(dead_code)]
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
    let curve_order = BigUint::parse_bytes(curve_order_bytes, 16).expect("failed to parse curve order");
    // convert the parent private key and the left half of the hmac result to Biguint for arithematic operations
    let parent_key_bigint = BigUint::from_bytes_be(key);
    let left_half_key_bigint = BigUint::from_bytes_be(left_half);
    // calculate = Add them together and take the modulo with the curve. the result is the child private key
    let child_private_key = (parent_key_bigint + left_half_key_bigint) % &curve_order;
    //create the Childkey struct instance
    let child_key = ChildKey {
        private_key: child_private_key.to_bytes_be(),
        chain_code: right_half.to_vec(),
    };

    serde_json::to_string(&child_key).map_err(|e| e.to_string())
}


// Given an extended private key and a BIP32 derivation path, compute the child private key found at the last path
// The derivation path is formatted as an array of (index: int, hardened: bool) tuples.
fn get_child_key_at_path(key: [u8; 32], chaincode: [u8; 32], paths: Vec<(u32, bool)>) -> ChildKey {
  
}







// json deserailized: {"version":[4,53,131,148],"depth":0,"parent_fingerprint":[0,0,0,0],"child_number":[0,0,0,0],"chain_code":[234,111,99,186,187,61,197,197,142,164,205,17,203,63,201,215,186,165,28,14,20,190,130,48,255,184,177,105,103,150,166,63],"key_data":[0,60,206,72,200,79,34,52,60,189,172,142,127,37,46,216,202,17,252,227,41,222,174,126,214,53,183,56,34,223,237,156,119]}