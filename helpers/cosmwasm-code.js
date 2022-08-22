import {SigningCosmWasmClient, CosmWasmClient} from "@cosmjs/cosmwasm-stargate";

var hiz = 'okay';
console.log('aloha hi from code', hiz);
if (hi) {
    console.log('again', hi);
}
// client = await SigningCosmWasmClient.connectWithSigner(rpc, alice)
// client = await CosmWasmClient.connect("http://127.0.0.1:26657")
// console.log('aloha client', client)
const foo = async () => {
    let client
    try {
        client = await CosmWasmClient.connect("http://127.0.0.1:26657")
    } catch (e) {
        console.error('Issue connecting to RPC', e)
    }
    console.log('in async')
    return await client.getHeight()
    // return await client.getHeight()
}

// This does not work, btw, just keep that in mind.
// Anything you want available in the REPL needs to be in the *-init.ts file
hi = 'this does not work'

foo().then((height) => {
    console.log('block height', height)
})

export default {
    foo
}