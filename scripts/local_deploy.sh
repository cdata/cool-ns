#!/bin/bash

CODE_ID=$( \
junod tx wasm store \
  ./artifacts/cool_ns.wasm \
  --from test-user-local \
  --gas 100000000 \
  -b block \
  --output json \
  -y | jq -r .logs[0].events[-1].attributes[0].value \
)

echo "Smart contract code ID is $CODE_ID"

CONTRACT_ADDRESS=$( \
junod tx wasm instantiate $CODE_ID \
  '{"allowed_tlds": ["cool","rad"]}' \
  --label coolns \
  --from test-user-local \
  -b block \
  --output json \
  -y | jq -r  .logs[0].events[0].attributes[0].value \
)

echo "Instance address is $CONTRACT_ADDRESS"