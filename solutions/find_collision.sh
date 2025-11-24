#!/bin/bash

if [ $# -ne 1 ]; then
    echo "Usage: $0 <input_string>"
    exit 1
fi

INPUT_STRING="$1"

if [ ${#INPUT_STRING} -ne 8 ]; then
    echo "Error: Input string must be exactly 8 characters"
    exit 1
fi

OUTPUT_STRING=""

# Swaps characters positions so when they are XORed the result is the same
for (( i=0; i<8; i++ )); do
    if [ $i -lt 4 ]; then
        OUTPUT_STRING="${OUTPUT_STRING}${INPUT_STRING:$((i+4)):1}"
    else
        OUTPUT_STRING="${OUTPUT_STRING}${INPUT_STRING:$((i-4)):1}"
    fi
done

echo "$OUTPUT_STRING"
