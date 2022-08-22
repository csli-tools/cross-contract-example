import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing"
import { calculateFee, GasPrice } from "@cosmjs/stargate"
import { CosmWasmClient, SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
const { StargateClient } = require('@cosmjs/stargate');
import { blue, yellow, red } from "chalk";
import * as fs from "fs";
import inquirer from 'inquirer';

const contractDetails = new Map([
    ['demo_totals', ''],
    ['dinner', ''],
    ['scholarship_list', ''],
]);

// const thisDirectory = path.dirname(fileURLToPath(import.meta.url));
// console.log('aloha thisDirectory', thisDirectory)
//
// const tsconfigPath = `${thisDirectory}/../tsconfig_repl.json`;
// import { foo } from `${thisDirectory}/cosmwasm-code`

// let wasmPath: string = 'will get updated in main()'
// const wasmPath = await import.meta.resolve('../whitelist-binary/cw1_whitelist.wasm')
// console.log('aloha wasmPath early on', wasmPath)

import * as path from 'path';
import { fileURLToPath } from 'url';

// let thisDirectory: string;
// thisDirectory = path.dirname(fileURLToPath(import.meta.url));
// console.log('aloha thisDirectory', thisDirectory)

// const tsconfigPath = `${thisDirectory}/../tsconfig_repl.json`;

const scholarshipsWasmPath = '../whitelist-binary/cw1_whitelist.wasm';
// TODO: use the dotenv NPM package and have a .env file (gitignored) and a .env.template
const rpc = "http://127.0.0.1:26657"
let client = {} as CosmWasmClient;
let signingClient = {} as SigningCosmWasmClient;

// We'll set these in main() and they'll be available in the cosmjs-cli REPL
let deployer: DirectSecp256k1HdWallet = {} as DirectSecp256k1HdWallet;
let alice: DirectSecp256k1HdWallet = {} as DirectSecp256k1HdWallet;
let bob: DirectSecp256k1HdWallet = {} as DirectSecp256k1HdWallet;
let jabroni: DirectSecp256k1HdWallet = {} as DirectSecp256k1HdWallet;
let reservation: DirectSecp256k1HdWallet = {} as DirectSecp256k1HdWallet;
let dinner: DirectSecp256k1HdWallet = {} as DirectSecp256k1HdWallet;
let scholarships: DirectSecp256k1HdWallet = {} as DirectSecp256k1HdWallet;

function get_deployer() {
  return {
    mnemonic: "tenant expect swarm radar casual hello labor hundred discover apology bitter rapid school skill lunch amazing live napkin exhaust vast damp lizard manual great",
    address0: "wasm1kr3q60whahcqr785x405y4qkc0lm8ywwu6wtkd",
  }
}

const aliceData = {
  mnemonic: "enlist hip relief stomach skate base shallow young switch frequent cry park",
  address0: "wasm14qemq0vw6y3gc3u3e0aty2e764u4gs5lndxgyk",
}

async function main() {
  alice = await DirectSecp256k1HdWallet.fromMnemonic(aliceData.mnemonic, { prefix: "wasm" })
  deployer = await DirectSecp256k1HdWallet.fromMnemonic(get_deployer().mnemonic, { prefix: "wasm" })
  // console.log('aloha deployer', deployer)

  // const client = await SigningCosmWasmClient.connectWithSigner(rpc, alice)
  // const client = await CosmWasmClient.connect("http://127.0.0.1:26657")
  // client.getHeight()
  // console.log('aloha client', client)

  // Upload contract
  // const wasm = fs.readFileSync(wasmPath)
  // const uploadFee = calculateFee(1_500_000, gasPrice)
  // const uploadReceipt = await client.upload(aliceData.address0, wasm, uploadFee)
  // console.info("Upload receipt:", uploadReceipt)

  // Instantiate
  // const instantiateFee = calculateFee(500_000, gasPrice)
  // // This contract specific message is passed to the contract
  // // The instantiate message can take an optional "owner", left out here.
  // const msg = {}
  // const { contractAddress } = await client.instantiate(
  //   aliceData.address0,
  //   uploadReceipt.codeId,
  //   msg,
  //   "your-label",
  //   instantiateFee,
  //   { memo: 'Can add a memo. This whole parameter is optional.' },
  // )
  // console.info(`Contract instantiated at: `, contractAddress)

  // Execute contract
  // const executeFee = calculateFee(300_000, gasPrice)
  // const result = await client.execute(aliceData.address0, contractAddress, { release: {} }, executeFee)
  // const wasmEvent = result.logs[0].events.find((e) => e.type === "wasm")
  // console.info("The `wasm` event emitted by the contract execution:", wasmEvent)
}

// @ts-ignore
await main()

export let hi = 'aloha'
// export default {
//   hi
// }

type FetchError = {
  "cause": {
    "errno": number,
    "code": string,
    "syscall": string,
    "address": string,
    "port": number
  }
}

const height = async () => {

  let currentHeight
  try {
    client = await CosmWasmClient.connect("http://127.0.0.1:26657")
    currentHeight = await client.getHeight()
  } catch (e) {
    const u = JSON.parse(JSON.stringify(e)); // a useful error object
    // console.log('error u', u)
    if (u.cause && u.cause.code && e.cause.code === 'ECONNREFUSED') {
      console.log(red(`Couldn't connect. Is your ${yellow('wasmd')} running? We're looking for a local port 26657`))
    } else {
      console.error('Issue connecting to RPC. Wanna file a ticket, ol\' buddy ol\' pal?', e.error.code)
    }
  }
  return currentHeight
}

const upload = async () => {
  // const wasmPath = await import.meta.resolve('../whitelist-binary/cw1_whitelist.wasm')
  // console.log('aloha wasmPath early on', wasmPath)
  inquirer
      .prompt([
        {
          name: 'contract',
          message: 'What size do you need?',
          type: 'list',
          choices: Array.from(contractDetails.keys()),
        },
      ]).then(async answer => {
    console.log('answer', answer)
    switch (answer.contract) {
      case 'demo_totals':
        // Upload contract
        signingClient = await SigningCosmWasmClient.connectWithSigner(rpc, deployer)
        // console.log('signingClient', signingClient)
        const wasm = fs.readFileSync('contracts/demo-totals/target/wasm32-unknown-unknown/release/cross_contract_demo_totals.wasm')
        const gasPrice = GasPrice.fromString("0.025stake")
        const uploadFee = calculateFee(1_500_000, gasPrice)
        signingClient.upload(get_deployer().address0, wasm, uploadFee).then(sheesh => {
          // I am embarrassed to have this nested .then, but not that embarrassed
          // Attachment leads to suffering
          console.info("Upload receipt:", sheesh)
        })
        break;
    }
  });

  // return 'we are done here'
  // console.log('contractToUpload', contractToUpload)

      // .then((answers) => {
      //   console.log('answers', answers)
      // })
      // .catch((error) => {
      //   if (error.isTtyError) {
      //     // Prompt couldn't be rendered in the current environment
      //   } else {
      //     console.log('error', error)
      //   }
      // });
  // let wasmPath
  // switch (contract) {
  //   case 's':
  //   case 'scholarships':
  //     wasmPath = scholarshipsWasmPath
  //     break;
  // }
  // signingClient = await SigningCosmWasmClient.connectWithSigner(rpc, alice)
  // let something = await signingClient.getHeight();
  // console.log('aloha something', something)
  // // const client = await CosmWasmClient.connect("http://127.0.0.1:26657")
  // // client.getHeight()
  // // console.log('aloha client', client)
  //
  // return true
}

const listValue = `
${yellow('Here are a couple things available:')}
- height() : returns current block height
- upload() : uploads the contract
`
console.log(blue(listValue))
console.log('⚛️  (you can type a command now, don\'t forget \'()\' )')
console.log('⚛️  (to see this again: l() )')
const l = () => {
  console.log(blue(listValue))
  console.log(`Please be a ` + `${yellow('good')} ` + `${blue('person')}.`)
  return true
}


// height().then((h) => {
//   console.log('done', h)
// })