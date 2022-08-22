# Cross-contract calls

This tutorial demonstrates cross-contract calls with [CosmWasm](https://github.com/CosmWasm/cosmwasm) v1.

# Overview

An end user calls the `reservation` contract to register for an upcoming dinner. The user will have to pay a little to get on the list, unless they've been given a scholarship and added to a special list. The special list is its own contract, and the list of registrants also lives in another contract.

This is a trivial example to demonstrate cross-contract calls.

# Contracts

There are three contracts here:

1. Reservation — this contract doesn't do much besides perform cross-contract calls to `dinner` contracts. It also stores how many times this contract has successfully registered people for dinner. 🍽️
2. Scholarship list — using the [whitelist contract](https://github.com/CosmWasm/cw-plus/tree/main/contracts/cw1-whitelist) from `cw-plus`, we keep a list of addresses that can register for dinner without paying. The compiled contract lives in the directory `whitelist-binary`.
3. Dinners — the `dinner` contract keeps track of dinner attendees. For demonstration purposes, each instance keeps track of only one dinner. For production, you may not want to implement it this way.

# Scenarios

Assuming there haven't been mistakes in deploying and initializing the three contracts, here are the success and failure scenarios.

## Success

A normal person calls the `reservation` contract attaching funds to pay for their attendance.

A person whose address exists on the scholarship contract calls the `reservation` contract, without attaching funds.

## Failure

A normal person calls the `reservation` contract but doesn't attach enough funds.

A person calls the `reservation` contract, but provides an invalid `dinner` contract address.

A registered person attempts to register again.

The dinner registration window has closed and a person tries to register.

## Starting `csli`

```sh
/Users/mike/Documents/csli/bin/cosmjs-cli.mjs --init cosmjs-cli-helpers/cosmwasm-init.ts --code cosmjs-cli-helpers/cosmwasm-code.ts
```