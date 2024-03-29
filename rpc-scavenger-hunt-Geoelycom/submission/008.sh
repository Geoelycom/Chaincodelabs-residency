# Which public key signed input 0 in this tx:
#   `e5969add849689854ac7f28e45628b89f7454b83e9699e551ce14b6f90c86163`
tx_id=e5969add849689854ac7f28e45628b89f7454b83e9699e551ce14b6f90c86163
# Get the transaction witness ARRAY with all its elements.
txinwitness_array=($(bitcoin-cli getrawtransaction $tx_id true | jq -r '.vin[0].txinwitness[]'))
# echo $txinwitness_array
# Get the transaction in witness hex. we need the hex to decode the script
hex=$(bitcoin-cli getrawtransaction $tx_id true | jq -r '.vin[0].txinwitness[2]')
# echo $hex
# use decodescript to decode the hex string in the transaction witness to take a futher look at the script
decoded_hex=$(bitcoin-cli decodescript $hex | jq -r '.asm' )

#*** BELOW IS THE BITCOIN SCRIPT from the output of the above command*****#
# OP_IF 
# 025d524ac7ec6501d018d322334f142c7c11aa24b9cffec03161eca35a1e32a71f 
# OP_ELSE 
# 144 
# OP_CHECKSEQUENCEVERIFY 
# OP_DROP 
# 02ad92d02b7061f520ebb60e932f9743a43fee1db87d2feb1398bf037b3f119fc2 
# OP_ENDIF 
# OP_CHECKSIG

# Explaining the Script
# If the condition for OP_IF is true, the public key 025d524ac7ec6501d018d322334f142c7c11aa24b9cffec03161eca35a1e32a71f is used.
# If the condition for OP_IF is false, and after OP_ELSE, another set of operations is executed, which includes a different public key 02ad92d02b7061f520ebb60e932f9743a43fee1db87d2feb1398bf037b3f119fc2.

# How do we determine which condition of the script is true?
# To know which branch of the script was run. this can be found in the tx_witness array.
# the first element in the txinwitness_array is the signature
# the second element in the determinate of which public key is used to signed the inputs
# The third element in the txinwitness-array is the public key in the script
# Define the public keys based on the script 
first_public_key="025d524ac7ec6501d018d322334f142c7c11aa24b9cffec03161eca35a1e32a71f"
second_pubkey="02ad92d02b7061f520ebb60e932f9743a43fee1db87d2feb1398bf037b3f119fc2"

# we need to check for the presence of '01' in the txinwitness array to determine the execution branch
if [[ " ${txinwitness_array[@]} " =~ " 01 " ]]; then
    echo $first_public_key
  else
    echo $second_pubkey
fi
