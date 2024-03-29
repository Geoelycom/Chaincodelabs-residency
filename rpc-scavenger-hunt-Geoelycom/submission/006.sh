# Which tx in block 257,343 spends the coinbase output of block 256,128?
block=256128

# use the getblockhash command to get the extra information relating to transaction from the block
get_block_info=$(bitcoin-cli getblockhash $block)

 # block hash = 0000000000000004f3fb306baa0638ffc181bc6b9752f9325612559c04d57bf9

# use the getblock to get information about the coinbase transaction. the coinbase transaction is the first transaction id from the array we get when we run getblock and pass the (hash of the block we are interested in)

coinbase_txid=$(bitcoin-cli getblock $get_block_info true | jq -r '.tx[0]')

# coinbase transaction id = 611c5a0972d28e421a2308cb2a2adb8f369bb003b96eb04a3ec781bf295b74bc

# Get block infomation(blockhash) of block 257,343
block257343_hash=$(bitcoin-cli getblockhash 257343)

# you should get block 256,128 to find it's coinbase TX, and then get block 257,343 to find the transaction spending it
block257343_details=($(bitcoin-cli getblock $block257343_hash true | jq -r '.tx[]'))
for txid in "${block257343_details[@]}"; do
unspent_txids=$(bitcoin-cli getrawtransaction "$txid" true | jq -r '.vin[] | select(.txid != null) | .txid')
# Check if any of the input transactions match the coinbase transaction of block 256128
for unspent_txid in $unspent_txids; do
 if [[ "$unspent_txid" == "$coinbase_txid" ]]; then
     echo $txid
     break
 fi
 done
done