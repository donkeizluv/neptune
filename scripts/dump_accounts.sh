#!/bin/bash

mkdir -p .account_bytes/accounts

dump_account() {
    local account_id=$1
    local output_file=".account_bytes/accounts/${account_id}"
    
    if [ ! -f "$output_file" ]; then
        echo "dumping account data ${account_id}..."
        solana account "$account_id" --output-file "$output_file" -u mainnet-beta
    else
        echo "account data ${account_id} already exists --> skip"
    fi
}

# List of accounts to download
accounts=(
    "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN"
    "CVMdMd79no569tjc5Sq7kzz8isbfCcFyBS5TLGsrZ5dN"
)

# Download each account
for account in "${accounts[@]}"; do
    dump_account "$account"
done
