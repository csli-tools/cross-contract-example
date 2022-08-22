# Whitelist contract

This directory contains `cw1_whitelist.wasm`, which was downloaded from the [`cw-plus` repository](https://github.com/CosmWasm/cw-plus).

It was taken from [the v0.13.4 release](https://github.com/CosmWasm/cw-plus/releases/tag/v0.13.4).

## Deploying locally

```shell
wasmd tx wasm store cw1_whitelist.wasm --from deployer --chain-id cc-23 --gas-prices 0.025stake --gas auto --gas-adjustment 1.3 --broadcast-mode block --output json -y
```
