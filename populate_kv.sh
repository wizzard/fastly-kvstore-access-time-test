#!/bin/bash
set -euo pipefail

if ! hash jq 2>/dev/null; then
    echo "'jq' command is not found. Please install 'jq' and re-run this script"
    exit 1
fi

STORE_NAME="PXKVstoreTEST"

store_id="$(fastly kv-store list -j | jq '.Data[] | select(.Name == "'$STORE_NAME'")'.StoreID)"
if [[ -n $store_id ]]; then
    echo "Error: Fastly KV Store ${STORE_NAME} already exists! Stopping the script."
    exit 1
fi

echo "Creating Fastly KV Store ${STORE_NAME}. In case of error please delete the store manually and re-run the script."
fastly kv-store create -n "$STORE_NAME"


STORE_ID="$(fastly kv-store list -j | jq -r '.Data[] | select(.Name == "'$STORE_NAME'")'.StoreID)"
if [[ -z $STORE_ID ]]; then
    echo "Error: Failed to get Fastly KV Store ID."
    exit 1
fi

echo "Fastly KV Store ID: ${STORE_ID}. Waiting for 5 seconds..."
sleep 5

echo "Adding Fastly KV Store Items..."
long_string=$(cat kv_data/long_string.txt)
data_json=$(cat kv_data/data.json)

fastly kv-store-entry create -s "$STORE_ID" -k "short_string" --value="TEST_123_TEST TEST 123!"
fastly kv-store-entry create -s "$STORE_ID" -k "u32_number" --value="1234567890"
fastly kv-store-entry create -s "$STORE_ID" -k "long_string" --value="$long_string"
fastly kv-store-entry create -s "$STORE_ID" -k "data_json" --value="$data_json"

echo "All done!"