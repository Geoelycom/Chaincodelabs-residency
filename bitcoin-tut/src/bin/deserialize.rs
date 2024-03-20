// use bitcoincore_rpc::bitcoin::base58;
use serde::Serialize;
use serde_json;
mod decode;
use decode::base58_decode;
fn main() {
  let base58_string = "tprv8ZgxMBicQKsPfCxvMSGLjZegGFnZn9VZfVdsnEbuzTGdS9aZjvaYpyh7NsxsrAc8LsRQZ2EYaCfkvwNpas8cKUBbptDzadY7c3hUi8i33XJ";
  match base58_decode(base58_string) {
    Ok(decoded_bytes) => {
       match deserialize_key(decoded_bytes) {
        Ok(json) => println!("json deserailized: {}", json),
        Err(e) => println!("Error deserializing key: {}", e),
       }
    },
   Err(e) => println!("Error: {}", e)
  }
}

#[derive(Serialize)]
pub struct ExtendedKey {
  // create a struct that will hold the data of our extended key
    version: [u8; 4],    // 4 bytes
    depth: u8,       // 1 byte
    parent_fingerprint: [u8; 4], // 4 bytes
    child_number: [u8; 4], // 4 bytes
    chain_code: [u8; 32], // 32 bytes
    key_data: Vec<u8>, // 33 bytes
}

// Deserialize the extended key bytes and return a JSON object
// https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#serialization-format
// 4 byte: version bytes (mainnet: 0x0488B21E public, 0x0488ADE4 private; testnet: 0x043587CF public, 0x04358394 private)
// 1 byte: depth: 0x00 for master nodes, 0x01 for level-1 derived keys, ....
// 4 bytes: the fingerprint of the parent's key (0x00000000 if master key)
// 4 bytes: child number. This is ser32(i) for i in xi = xpar/i, with xi the key being serialized. (0x00000000 if master key)
// 32 bytes: the chain code
// 33 bytes: the public key or private key data (serP(K) for public keys, 0x00 || ser256(k) for private keys)
 pub fn deserialize_key(bytes: Vec<u8>) -> Result<String, String> {    
  if bytes.len() != 78 {
   return Err("Invalid byte length of an extended key".to_string());
  } 
   let version = bytes[0..4].try_into().map_err(|_| "failed to extract version".to_string())?;
   let depth = bytes[4];
   let parent_fingerprint = bytes[5..9].try_into().map_err(|_| "failed to extract parent fingerprint".to_string())?;
   let child_number = bytes[9..13].try_into().map_err(|_| "failed to extract child number".to_string())?;
   let chain_code = bytes[13..45].try_into().map_err(|_| "failed to extract chain_code".to_string())?;
   let key_data =  bytes[45..78].to_vec();
 
  let ext_key = ExtendedKey {
    version,
    depth,
    parent_fingerprint,
    child_number,
    chain_code,
    key_data,
 
  };
 
  serde_json::to_string(&ext_key).map_err(|e| e.to_string())
 
 }