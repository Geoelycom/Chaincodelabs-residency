extern crate bitcoincore_rpc;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use bs58;
use hex;
use hmac_sha512::HMAC;
use serde::Serialize;
use serde_json;
use num_bigint::BigUint; // for modulus math on large numbers
use ripemd::{Ripemd160};
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256, Digest};
use std::error::Error;
use std::io::Read;
use std::path::PathBuf;
use std::str;

fn main(){
  let base58_string = "tprv8ZgxMBicQKsPfCxvMSGLjZegGFnZn9VZfVdsnEbuzTGdS9aZjvaYpyh7NsxsrAc8LsRQZ2EYaCfkvwNpas8cKUBbptDzadY7c3hUi8i33XJ";
  match base58_decode(base58_string) {
    Ok(decoded_bytes) => {
      println!("decoded bytes: {:?}", decoded_bytes);
    }, 
    Err(e) => {
      println!("Error decoding base58: {}", e)
   }
  }

}

#[derive(Serialize)]
pub struct ExtendedKey {
  // create a struct that will hold the data of our extended key
   pub version: [u8; 4],    // 4 bytes
   pub depth: u8,       // 1 byte
   pub parent_fingerprint: [u8; 4], // 4 bytes
   pub child_number: [u8; 4], // 4 bytes
   pub chain_code: [u8; 32], // 32 bytes
   pub key_data: Vec<u8>, // 33 bytes
}


// Decode a base58 string into an array of bytes
pub fn base58_decode(base58_string: &str) ->Result<Vec<u8>, String> {
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




// [4, 53, 131, 148, 0, 0, 0, 0, 0, 0, 0, 0, 0, 234, 111, 99, 186, 187, 61, 197, 197, 142, 164, 205, 17, 203, 63, 201, 215, 186, 165, 28, 14, 20, 190, 130, 48, 255, 184, 177, 105, 103, 150, 166, 63, 0, 60, 206, 72, 200, 79, 34, 52, 60, 189, 172, 142, 127, 37, 46, 216, 202, 17, 252, 227, 41, 222, 174, 126, 214, 53, 183, 56, 34, 223, 237, 156, 119, 156, 67, 8, 37]

//     let expected_hex = "04358394000000000000000000ea6f63babb3dc5c58ea4cd11cb3fc9d7baa51c0e14be8230ffb8b1696796a63f003cce48c84f22343cbdac8e7f252ed8ca11fce329deae7ed635b73822dfed9c779c430825";
// re-append the checksum to the data
  // If checksum is okay, reconstruct the full byte array including the checksum
  // let mut full_bytes = Vec::from(data); // convert slice to vector
  // full_bytes.extend_from_slice(original_checksum); // append the checksum
