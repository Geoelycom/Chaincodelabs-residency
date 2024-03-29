# How many new outputs were created by block 123,456?

# Block height to analyze
block_height="123456"
# # Get the block hash before retrieving block information
block_hash=$(bitcoin-cli getblockstats $block_height)
echo $block_hash | jq -r '.outs'









