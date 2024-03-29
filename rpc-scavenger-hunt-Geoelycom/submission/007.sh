# Only one single output remains unspent from block 123,321. What address was it sent to?
unspent_block=123321
get_block_hash=$(bitcoin-cli getblockhash 123321)

txids_array=($(bitcoin-cli getblock $get_block_hash true | jq -r '.tx[]'))
 # echo $txids_array

for txid in "${txids_array[@]}"; do
 raw_tx=$(bitcoin-cli getrawtransaction "$txid" true | jq -r '.hex')
 n_vals=$(bitcoin-cli decoderawtransaction "$raw_tx" true | jq -r '.vout[].n')
  for n in $n_vals; do
  address=$(bitcoin-cli gettxout $txid $n true | jq -r '.scriptPubKey.address')
  echo $address
  done
done