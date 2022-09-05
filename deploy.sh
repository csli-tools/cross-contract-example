#!/bin/bash
set -e

# In case of M1 MacBook use workspace-optimizer-arm64 instead of workspace-optimizer
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.6

NODE="--node https://rpc.uni.juno.deuslabs.fi:443"
TXFLAG="--node https://rpc.uni.juno.deuslabs.fi:443 --chain-id uni-3 --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.3 --broadcast-mode block"

if [ -z "$1"]
then
    OWNER=cross-contract-example-owner
    junod keys show $OWNER 2> /dev/null || junod keys add $OWNER
    JSON=$(jq -n --arg addr $(junod keys show -a $OWNER) '{ denom:"ujunox","address":$addr}') && \
        curl -X POST --header "Content-Type: application/json" --data "$JSON" https://faucet.uni.juno.deuslabs.fi/credit && echo
else 
    OWNER=$1
fi

ALICE=alice
BOB=bob

junod keys show $ALICE 2> /dev/null || junod keys add $ALICE
JSON=$(jq -n --arg addr $(junod keys show -a $ALICE) '{ denom:"ujunox","address":$addr}') && \
    curl -X POST --header "Content-Type: application/json" --data "$JSON" https://faucet.uni.juno.deuslabs.fi/credit && echo
junod keys show $BOB 2> /dev/null || junod keys add $BOB
JSON=$(jq -n --arg addr $(junod keys show -a $BOB) '{ denom:"ujunox","address":$addr}') && \
    curl -X POST --header "Content-Type: application/json" --data "$JSON" https://faucet.uni.juno.deuslabs.fi/credit && echo

# Store code
# In case of M1 MacBook replace demo_total.wasm with demo_total-aarch64.wasm
RES_DEMO_TOTAL=$(junod tx wasm store artifacts/cross_contract_demo_totals.wasm --from $OWNER $TXFLAG -y --output json -b block)
CODE_DEMO_TOTAL=$(echo $RES_DEMO_TOTAL | jq -r '.logs[0].events[-1].attributes[0].value')

RES_DINNER=$(junod tx wasm store artifacts/cross_contract_dinner.wasm --from $OWNER $TXFLAG -y --output json -b block)
CODE_DINNER=$(echo $RES_DINNER | jq -r '.logs[0].events[-1].attributes[0].value')

RES_WHITELIST=$(junod tx wasm store contracts/whitelist-binary/cw1_whitelist.wasm --from $OWNER $TXFLAG -y --output json -b block)
CODE_WHITELIST=$(echo $RES_WHITELIST | jq -r '.logs[0].events[-1].attributes[0].value')

# Instantiate
INIT_WHITELIST='{"admins":["'$(junod keys show -a $ALICE)'"], "mutable": true}'
junod tx wasm instantiate $CODE_WHITELIST "$INIT_WHITELIST" --from $OWNER --label "scholarship-list" $TXFLAG -y --no-admin
CONTRACT_WHITELIST=$(junod query wasm list-contract-by-code $CODE_WHITELIST $NODE --output json | jq -r '.contracts[-1]')

INIT_DEMO_TOTAL='{"denom":"ujunox"}'
junod tx wasm instantiate $CODE_DEMO_TOTAL "$INIT_DEMO_TOTAL" --from $OWNER --label "demo-total" $TXFLAG -y --no-admin
CONTRACT_DEMO_TOTAL=$(junod query wasm list-contract-by-code $CODE_DEMO_TOTAL $NODE --output json | jq -r '.contracts[-1]')

INIT_DINNER='{"denom":"ujunox", "scholarship_address":"'$CONTRACT_WHITELIST'"}'
junod tx wasm instantiate $CODE_DINNER "$INIT_DINNER" --from $OWNER --label "dinner" $TXFLAG -y --no-admin
CONTRACT_DINNER=$(junod query wasm list-contract-by-code $CODE_DINNER $NODE --output json | jq -r '.contracts[-1]')

# ALICE and BOB call CONTRACT_DEMO_TOTAL
REGISTER_WITH_PAYMENT='{"register_with_payment":{"dinner_contract":"'$CONTRACT_DINNER'"}}'
junod tx wasm execute $CONTRACT_DEMO_TOTAL "$REGISTER_WITH_PAYMENT" --amount 10000ujunox --from $BOB $TXFLAG -y

REGISTER_WITH_SCHOLARSHIP='{"register_with_scholarship":{"dinner_contract":"'$CONTRACT_DINNER'"}}'
junod tx wasm execute $CONTRACT_DEMO_TOTAL "$REGISTER_WITH_SCHOLARSHIP" --from $ALICE $TXFLAG -y

# BOB shouldn't be able to register with scholarship
# REGISTER_WITH_SCHOLARSHIP='{"register_with_scholarship":{"dinner_contract":"'$CONTRACT_DINNER'"}}'
# junod tx wasm execute $CONTRACT_DEMO_TOTAL "$REGISTER_WITH_SCHOLARSHIP" --from $BOB $TXFLAG -y
