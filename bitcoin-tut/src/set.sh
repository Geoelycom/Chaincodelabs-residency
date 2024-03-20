#!/bin/bash
# rpc_user="user_030"
# rpc_password=3p8HzQbAModM
# rpcconnect=35.209.148.157
# rpc_port=8332
# block_height="654321"
# #use bitcoin-cli to get the block hash
# block_hash=$(bitcoin-cli -rpcuser=$rpc_user -rpcpassword=$rpc_password -rpcport=$rpc_port getblockhash $block_height)

# echo  "Block hash for block $block_height:$block_hash"
# 000000000000000000058452bbe379ad4364fe8fda68c45e299979b492858095
# echo "Total Output Value in Block $block_height: $total_outputs BTC"new_outputs=$(($new_outputs + $(echo "$raw_transaction" | jq -r '[.vout[] | select(.spentTxID == null)] | length' ))
# filter out transaction outputs and count new ones

echo $new_outputs




"vin": [
    {
      "scriptSig": {
        "asm": "",
        "hex": ""
      },
      "sequence": 4294967294,
      "txid": "c9da5e51de40985b8e29d9ddd11b5d8818250773e3f3129bf065900ee841f6b5",
      "txinwitness": [
        "304402200dd758801b40393f68dad8ab57558803efcd2b681ee31eb44fb3cfa9666d2bf90220254d34fa4990e23652bf669053c5e16fd2fbb816bed2eeb44c1f1e6e54143e3e01",
        "02bbb4ba3f39b5f3258f0014d5e4eab5a6990009e3e1dba6e8eaff10b3832394f7"
      ],
      "vout": 0
    },
    {
      "scriptSig": {
        "asm": "",
        "hex": ""
      },
      "sequence": 4294967294,
      "txid": "44fbdaec8b794fa0b6e9ff70d76d4464ada38fe5aeac7285f16fbbaf86894fd5",
      "txinwitness": [
        "304402201694761a5749b6a84f71459c04a44cf9d34a36ae8c9044c3af7a3a5514ef2e64022058f61feb92d6d54b71fdea47e7dfcd20f6a5c12e2fbcb15bc44fe95c73f2e80801",
        "03aaf17b1a7b4108f7e5bc4f7d59c20f7fb1a72dbc74a9a3d6d1f8488df159c760"
      ],
      "vout": 0
    },
    {
      "scriptSig": {
        "asm": "",
        "hex": ""
      },
      "sequence": 4294967294,
      "txid": "c5263cf8798dd44f4aeae6421a1b5ab6a26d8623d7294c5487eec3b162305e47",
      "txinwitness": [
        "3044022014b65c60f65e62d9dac893e404c8de2a007c7c6b74dbac18e454d8374e159759022012453f69112adadf9495fd3fe288aa5ed9e3d836340da06fa1e82c8e09adef5701",
        "03a6d919c76d9117c23570a767450013edf31cf6be7d3b5a881c06a9aa1f2c24ce"
      ],
      "vout": 0
    },
    {
      "scriptSig": {
        "asm": "",
        "hex": ""
      },
      "sequence": 4294967294,
      "txid": "09f87c09c96058b8b72ed0caefff37fbefd9d4ebe64876d3df93cc2e358763a5",
      "txinwitness": [
        "304402203d3b02390803c1d673fa49bd64d4a26fbeb29e3fc152af8f844d776c9409e41302206903a011a04e00a7f4ec606da4320226d2d393f565702cc58cfcef6dca67f84c01",
        "0383d12258e3e294a6d7754336f6b4baef992ec4b91694d3460bcb022b11da8cd2"
      ],
      "vout": 0
    }
  ]



  "vout": [
    {
      "n": 0,
      "scriptPubKey": {
        "address": "bc1qspcwcw25anwtlsss6qgharfg5x0ts3njad8uve",
        "asm": "0 8070ec3954ecdcbfc210d0117e8d28a19eb84672",
        "desc": "addr(bc1qspcwcw25anwtlsss6qgharfg5x0ts3njad8uve)#pzjnvw8p",
        "hex": "00148070ec3954ecdcbfc210d0117e8d28a19eb84672",
        "type": "witness_v0_keyhash"
      },
      "value": 0.01000698
    },
    {
      "n": 1,
      "scriptPubKey": {
        "address": "bc1qkhlyd3j8x5lvnsrrw3j42qsfgz2lq2yu3cs5lr",
        "asm": "0 b5fe46c647353ec9c06374655502094095f0289c",
        "desc": "addr(bc1qkhlyd3j8x5lvnsrrw3j42qsfgz2lq2yu3cs5lr)#hzcalwww",
        "hex": "0014b5fe46c647353ec9c06374655502094095f0289c",
        "type": "witness_v0_keyhash"
      },
      "value": 0.0823
    }
  ]


  bitcoin-cli createmultisig 1 "[\"02bbb4ba3f39b5f3258f0014d5e4eab5a6990009e3e1dba6e8eaff10b3832394f7\",
\"03aaf17b1a7b4108f7e5bc4f7d59c20f7fb1a72dbc74a9a3d6d1f8488df159c760\",
\"03a6d919c76d9117c23570a767450013edf31cf6be7d3b5a881c06a9aa1f2c24ce\",
\"0383d12258e3e294a6d7754336f6b4baef992ec4b91694d3460bcb022b11da8cd2\"]"


# Looping over a transaction and setting it into a bash array

tx_ids=($(jq -r '.tx|@sh' <<<$(bitcoin-cli getblock $(jq -r '.blockhash' <<<$(bitcoin-cli getblockstats 321123))) | tr -d "\'"))

for tx in "${tx_ids[@]}"; do
  bitcoin-cli getrawtransaction $tx
  echo
done;


# vinout of block 256128 = vout": 
#  [
#     {
#       "n": 0,
#       "scriptPubKey": {
#         "address": "1HtUGfbDcMzTeHWx2Dbgnhc6kYnj1Hp24i",
#         "asm": "OP_DUP OP_HASH160 b93dfd929a473f652c7c3e73ed093d60ae6385c3 OP_EQUALVERIFY OP_CHECKSIG",
#         "desc": "addr(1HtUGfbDcMzTeHWx2Dbgnhc6kYnj1Hp24i)#qfrrzecw",
#         "hex": "76a914b93dfd929a473f652c7c3e73ed093d60ae6385c388ac",
#         "type": "pubkeyhash"
#       },
#       "value": 25.41786912
#     }
#   ],

tx": [
    "90d10b04417f2698fa8fed92ca5c951d26613a1737da69a7450f8c7706ba8783",
    "67aa86bfac08ef9e520b9c07f9062c332552d9654022932bd9b45daceff7c3d2",
    "c54714cb1373c2e3725261fe201f267280e21350bdf2df505da8483a6a4805fc",
    "1361ec1190744303d7950aeaeb4bc5e6e50c6f6eb8160cc01f0f38713097c39b",
    "4785bd0e3eb810436ae9c07e631d8fbd16201fe043d62b1148b91850779b9172",
    "bba17a53809d234788560cb712796301e02f6382b12bb16250eedda586f64ffb",
    "ab2d453847c965115c3237f6882ba8f7598e7c32329188afd6863df6db1862a5",
    "f5fcb2b8882753e74d704f17ac5be6d286d7e418549a2954923bdceedf26f47e",
    "ccab1bf21396e184a817ab8e4bb8f576e627b6b9ffc2a247de6a7598709db4d8",
    "71421ee33dbc171d0bf579352473900e0ca280514f7a0dbd3a2b89cd7189f811",
    "e2da8c5cc833da3f143a8ca274b51321c444dfb949ece33d2c61f500a562310b",
    "bab766c65c9ce41c39f33c4b939f5825102acde9553771a85ee37e5458226dd4",
    "94d2a974dd86a772db27faa22ac39617e66cbb3aeb68685af7829a16031f47df",
    "c842b2a9f8657585d38f5b360a2f4ddc2759ad2e6c265a42467a6fcfc61dc732",
    "285070f6c96ea6ed603f3779b8c7a0558785773d611107ca102c532ca9da3da9",
    "8b37bacffe4980d9a97c0c7be15ad45dcdef32144f8678a47c850e0c87c36453",
    "b41fea1bad2f9045a16424c1decf13ea61085c7da9d41da9d2bf8fb6a4eec32a",
    "20ab5926922b1ff675c17afd3bfbfb4ec4118d2dc6079aa0c2ed39151d47932c",
    "a8bc4a5783cfcaaf832dc65b21f44cf6a99450ce68acadb6256c746a5061eec8",
    "10d2a1463d6a6237068fc789ce82d9acd78be71f03ee083eea3445137fba3f24",
    "3247fac5a8ddc104cf6dfbe52aefb7c67fe30ad682680ef13538860325f42e31",
    "70092b05e7481cb760ee2fdbe5dd0adb72568c6a44cafb82942a78107dd18c43",
    "259e3918490695aceb121c079a06a25e1442b36fa6dd166d1c2fc571a2e4326b",
    "1e0521b6a93f6eff12d923609192298d13fb99874181f158f1b796faa68dc254",
    "e83cd6d321881908729edc5fc2f3242467c46ab41501da4d8cf2d455ed68d750",
    "208f125ea7d63a0c7488ffc6fd8215e3c508e852575ece1a05a7c6101e21ef7d",
    "74c0a7c6c636925be2d4d51c676aa3c49cccb7b8ee2f94b0b51caa75829b59da",
    "5c8741ba41bbced08e91f07388babb1262773c6955baf9a17e2a7e6db0bd6a38",
    "0883e294f08849aca14d4b30316d09abb072e46bc5e6bf52aed79220f4f74ba9",
    "3bfaadcd3bc6f31836edc2f14081d224b3ad4dee880ac4c56942cd13146c5f3d",
    "b143250ca5d0f3706344b126c9c1710831919c1910c10d223cd7549fbbb4b4a4",
    "3e68926fad063cbdb150d1e811071d052bbe146b11bef688d4ca64136bc7a802",
    "89d9d3c9a9ed100f3c464f17a0fb5455893dd8345eab2f00dd2a6dd7b6da679d",
    "2410a9ffd008bd98ed0eb11e8f13c1e5f8f7080b2852b4e096c5d5fad6328d4c",
    "5ae3a47c5a58c7680e8de9731c3a61492530bac3e61af1dc323c0619730c1509",
    "7807fa19b6e47d24f9cdb5cfb983cd6c9cd8855d8b5bd7833f85ea7d503ed714",
    "acf13f990e3630d3512da333c86a1f1735bad7785c949aa09bbf63edd6c164c0",
    "1534292ccd4792e0ea08c0fa1108476e565aa0ecc05783e79aca8f70b527e025",
    "d6c8f6de694fe42a92220fc3d1d816f7adf15cfd14030297098c824507a46a10",
    "62a314755a8c037dbd77aa1ad76c065aed6e3e6f87ebfcfcc75bf1dc328236e0",
    "7f49463b5956891d18010754a90861c4981906368ad421a70ccafc3d1eaaf1f5"
  ],


007

#   filtered_json=$(echo "$transaction_array" | jq -c '.[] | select((.vin[]? | .vout > 1))')
# address=$(echo "$filtered_json" | jq -r '.vout[0].scriptPubKey.address') 
# echo "$address"


# txarray_string=$(echo "$block257343_details" | jq -r '@csv' | tr -d '"')

# echo $txarray_string
# Convert the string to a Bash array
# IFS=', ' read -r -a txarray <<< "$txarray_string"

# Save the array in a variable
# saved_txarray=("${txarray[@]}")

# convert json array to bash
# IFS=', ' read -r -a txarray <<< "$(echo "$block257343_details" | jq -r '.[]')"



"vin": [
    {
      "scriptSig": {
        "asm": "3045022100f2e1c84d221f1e383d11f99037dc50acd26bf651943e5463a6f480c233a8135802201687649906ab3b3dd5c1e6933631196607f9e4af4780b5d0a1b5cafcd5cd7a88[ALL] 04d53d66737d24d2a899c92ddf367e77c91d8690c483c151ce25d03e91eccc57f801291812487b0a16513a9720835584feccdb39f4091dd7894b8761f7aac922be",
        "hex": "483045022100f2e1c84d221f1e383d11f99037dc50acd26bf651943e5463a6f480c233a8135802201687649906ab3b3dd5c1e6933631196607f9e4af4780b5d0a1b5cafcd5cd7a88014104d53d66737d24d2a899c92ddf367e77c91d8690c483c151ce25d03e91eccc57f801291812487b0a16513a9720835584feccdb39f4091dd7894b8761f7aac922be"
      },
      "sequence": 4294967295,
      "txid": "00c9547b693651c2eefadd22128ca07c3674bc2dd6bc1f0a70b652e0b96b31d8",
      "vout": 1
    },
    {
      "scriptSig": {
        "asm": "3045022100a1d5dc09738c89867fd6fb821d2cf0778e958f92f8926ca9c95b1bc9c90888f00220430ee4fed6702e4e06f990527c0dd88e3503f8173ac2e758617d5213956d9de6[ALL] 0413a4a8180c086ed8ad5c6dfb38ab43de6f39eaa48faf40393fdaae6591c1bd8d24890a5f437444e24ce958dccd6fbf246f25831c1feb0409d2bf62d081369953",
        "hex": "483045022100a1d5dc09738c89867fd6fb821d2cf0778e958f92f8926ca9c95b1bc9c90888f00220430ee4fed6702e4e06f990527c0dd88e3503f8173ac2e758617d5213956d9de601410413a4a8180c086ed8ad5c6dfb38ab43de6f39eaa48faf40393fdaae6591c1bd8d24890a5f437444e24ce958dccd6fbf246f25831c1feb0409d2bf62d081369953"
      },
      "sequence": 4294967295,
      "txid": "fba53c771267b742d590bde2d8a6c9a073a8c2ba97636cf1a822586e1c4a43e0",
      "vout": 1
    }
  ],



  extern crate bitcoincore_rpc;
use bitcoincore_rpc::{Auth, Client, RpcApi};

fn main() {

    let rpc = Client::new("http://localhost:8332",
                          Auth::UserPass("geoelycom".to_string(),
                                         "Beautifulgirl456".to_string())).unwrap();
    let best_block_hash = rpc.get_best_block_hash().unwrap();
    let get_blockchain_info = rpc.get_blockchain_info().unwrap();
// Get a random transaction ID
    let transaction_id = "466200308696215bbc949d5141a49a4138ecdfdfaa2a8029c1f9bcecd1f96177".to_string();

    // first, retrieve the raw transaction in hex
   let raw_tx = rpc.get_raw_transaction(&transaction_id, None).unwrap();
   // decode the raw transaction hex into a json Object
   let decode_raw_tx = rpc.decode_raw_transaction(&raw_tx).unwrap();

    println!("get_blockchain_info: {}", best_block_hash);
    println!("get_blockchain_info: {:?}", get_blockchain_info);
}


## Decoded xpub 
[4, 53, 131, 148, 0, 0, 0, 0, 0, 0, 0, 0, 0, 234, 111, 99, 186, 187, 61, 197, 197, 142, 164, 205, 17, 203, 63, 201, 215, 186, 165, 28, 14, 20, 190, 130, 48, 255, 184, 177, 105, 103, 150, 166, 63, 0, 60, 206, 72, 200, 79, 34, 52, 60, 189, 172, 142, 127, 37, 46, 216, 202, 17, 252, 227, 41, 222, 174, 126, 214, 53, 183, 56, 34, 223, 237, 156, 119, 156, 67, 8, 37]