// Derive the secp256k1 compressed public key from a given private key
// BONUS POINTS: Implement ECDSA yourself and multiply your key by the generator point!
// use secp256k1::{Secp256k1, SecretKey, PublicKey};

// pub fn derive_public_key_from_private(key: &[u8; 32]) -> [u8; 33] {
//   //create a Secp256k1 context object
// let secp = Secp256k1::new();
// // create a secret key object
// let secret_key = SecretKey::from_slice(&[])

// }













// JSON representation of  deserialized base58 bytes array
//     json_rep =  { 
//   "version":[4,53,131,148],
//   "depth":0,
//   "parent_fingerprint":[0,0,0,0],
//   "child_number":[0,0,0,0],
//   "chain_code":[234,111,99,186,187,61,197,197,142,164,205,17,203,63,201,215,186,165,28,14,20,190,130,48,255,184,177,105,103,150,166,63],
// "key_data":[0,60,206,72,200,79,34,52,60,189,172,142,127,37,46,216,202,17,252,227,41,222,174,126,214,53,183,56,34,223,237,156,119]