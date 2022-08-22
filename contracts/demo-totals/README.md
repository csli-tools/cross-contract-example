# Demo totals

This smart contract is the first contract to be called in this tutorial demonstrating cross-contract calls. This is a mostly-useless contract that you'd likely not architect this way, but is good for demonstrative purposes. We will call this smart contract, and the only purpose is to keep a count of the total number of successful dinner registrations we've helped. Besides that total, this contract only functions to use cross-contract calls to ferry information to other dinner-related contracts and properly act on the reply that is received.

## Building

If you have a linux-like terminal you may run:

    ./build.sh

## Deploying

We'll cover how to deploy this to a local simulation as well as to Juno testnet.

### Simulate blockchain

We'll be simulating these contracts using [simd](). To do this:

Ensure you have Golang installed, might as well use 1.18. Follow directions here:
https://golang.org/doc/install

When you run:

    go version

You shouldn't see an error and should see something like:

```sh
go version go1.18.4 darwin/amd64
```

Get the Cosmos SDK:

```sh
git clone https://github.com/cosmos/cosmos-sdk.git
cd cosmos-sdk
make build
```

Once it successfully builds, ensure there's a `simd` binary should exist somewhere like: `/Users/friend/go/bin/simd` and you should be able to run the command:

    simd version

and see results without an error.

If you've run simd before, you may need to reset your database with:

    simd unsafe-reset-all

Initialize the simulation:

```sh
simd init cross-contract --chain-id cc-19
simd keys add deployer
simd add-genesis-account deployer 10000000000000000000000000stake
simd gentx deployer 1000000000000000stake --chain-id cc-19
simd collect-gentxs
simd start
```

Or for Juno using `junod`:

```sh
#junod unsafe-reset-all
junod init cross-contract --chain-id cc-23
junod keys add deployer
junod keys show
# Copy the address and replace the Juno address in the next command
junod add-genesis-account juno1hemfted3mgj04hae507lwhm8xuj33hqjg6y9q8 10000000000000000000000000stake
junod gentx deployer 1000000000000000stake --chain-id cc-23
junod collect-gentxs
junod start
```

```sh
wasmd unsafe-reset-all
wasmd init cross-contract --chain-id cc-23
wasmd keys add deployer
wasmd keys show deployer
# Copy the address and replace the Juno address in the next command
wasmd add-genesis-account wasm1fm3jajgrz88llawp5clfcfld9t0y82vpn48l8h 10000000000000000000000000stake
wasmd gentx deployer 1000000000000000stake --chain-id cc-23
wasmd collect-gentxs
wasmd start
```

You should be seeing a screen with chunks of logs that flow every 6 seconds or so.

Now it's time to deploy the contracts. We're going to use [cosmjs-cli](https://www.npmjs.com/package/@cosmjs/cli) for this.

You can install this globally with:

    npm install -g @cosmjs/cli

Check it's working with:

    cosmjs-cli --version

This is a command-line interface (CLI) that will allow us to have a [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop), or prompt where you can run stuff.

Note the `helpers` directory in this project. It contains the file we want to use:

    cd helpers

In this folder we'll see a TypeScript helper file that we'll feed into `cosmjs-cli`.

If your IDE has a hard time navigating the declarations or inspecting functions, make sure you run this command in the parent directory:

    npm i

(We technically don't need these dependencies, since the `cosmjs-cli` has probably imported all of those packages, but it can help your developer experience if you'd like to modify the file, explore the codebase, or write new helper files.)

## Deploying locally

```shell
wasmd tx wasm store target/wasm32-unknown-unknown/release/cross_contract_reservation.wasm --from deployer --chain-id cc-23 --gas-prices 0.025stake --gas auto --gas-adjustment 1.3 --broadcast-mode block --output json -y
```

## Troubleshooting hints

When running `make build` it's not unusual for something to go wrong. Keep in mind that you might want to check your `~/.bashrc`, `~/.zshrc`, etc. It's possible you have build flags that could interfere here, like the `LDFLAGS` environment variable being set.