#!/bin/bash

set -euo pipefail

usage() {
    echo "Usage: $0 <target_hash>" >&2
    echo "Example: $0 1b575451" >&2
    exit 1
}

validate_hash() {
    [[ "$1" =~ ^[0-9a-fA-F]{8}$ ]] || { echo "Error: Hash must be 8 hex characters" >&2; exit 1; }
}

extract_bytes() {
    local hash=$1
    local -n result=$2
    local num_bytes=$((${#hash} / 2))
    
    for i in $(seq 0 $((num_bytes - 1))); do
        local offset=$(((num_bytes - 1 - i) * 2))
        result[$i]=$((0x${hash:$offset:2}))
    done
}

find_xor_pair() {
    local target=$1
    local -n char1_ref=$2
    local -n char2_ref=$3
    
    ## 33 to 126 are printable/typeable and avoids space
    for c1 in $(seq 33 126); do
        for c2 in $(seq 33 126); do
            if [[ $((c1 ^ c2)) -eq $target ]]; then
                char1_ref=$c1
                char2_ref=$c2
                return 0
            fi
        done
    done
    return 1
}

append_char() {
    local -n str=$1
    str+=$(printf "\\$(printf %03o $2)")
}

main() {
    [[ $# -ne 1 ]] && usage
    
    local target_hash=$1
    validate_hash "$target_hash"
    
    local -a bytes
    extract_bytes "$target_hash" bytes
    
    local solution=""
    local -a first_chars
    local -a second_chars
    
    for i in "${!bytes[@]}"; do
        local char1 char2
        find_xor_pair ${bytes[$i]} char1 char2 || { echo "Error: No XOR pair for byte $i" >&2; exit 1; }
        first_chars[$i]=$char1
        second_chars[$i]=$char2
    done
    
    for char in "${first_chars[@]}"; do
        append_char solution $char
    done
    
    for char in "${second_chars[@]}"; do
        append_char solution $char
    done
    
    echo -n "$solution"
}

main "$@"
