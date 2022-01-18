#!/bin/bash

TX=$( \
junod tx wasm store \
  ./artifacts/cool_ns.wasm \
  --from test-user \
  --chain-id testing \
  --gas auto \
  --output json \
  -y | jq -r '.txhash'
)

CODE_ID=$(junod query tx $TX --output json | jq -r '.logs[0].events[-1].attributes[0].value')

echo $CODE_ID