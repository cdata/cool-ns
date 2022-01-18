#!/bin/bash

docker run -it \
  -p 26656:26656 \
  -p 26657:26657 \
  ghcr.io/cosmoscontracts/juno:latest \
  ./setup_and_run.sh juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y
