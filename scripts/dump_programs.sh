#!/bin/bash

mkdir -p .account_bytes/programs

dump_program() {
    local program_id=$1
    local custom_name=$2
    local output_file=".account_bytes/programs/${custom_name}"
    
    if [ ! -f "$output_file" ]; then
        echo "dumping program ${program_id}..."
        solana program dump "$program_id" "$output_file" -u testnet
    else
        echo "program ${program_id} already exists --> skip"
    fi
}

# List of accounts to download - format: [program_id custom_filename]
programs=(
    "2Tp4S4CBn8osgs77BR6o1SVhRpiihWjG2kcsKc9ekPKC locked_voter.so"
)

# Download each program
for program in "${programs[@]}"; do
    read -r program_id custom_name <<< "$program"
    dump_program "$program_id" "$custom_name"
done
