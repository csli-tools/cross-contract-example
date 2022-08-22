"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
exports.__esModule = true;
exports.hi = void 0;
var proto_signing_1 = require("@cosmjs/proto-signing");
var stargate_1 = require("@cosmjs/stargate");
var StargateClient = require('@cosmjs/stargate').StargateClient;
var wasmPath = '../whitelist-binary/cw1_whitelist.wasm';
var rpc = "http://127.0.0.1:26657";
var client = {};
var deployer = {};
var alice = {};
var bob = {};
var jabroni = {};
var reservation = {};
var dinner = {};
var scholarships = {};
function get_deployer() {
    return {
        mnemonic: "stomach enlist hip relief skate base shallow young switch frequent cry park",
        address0: "wasm14qemq0vw6y3gc3u3e0aty2e764u4gs5lndxgyk",
        address1: "wasm1hhg2rlu9jscacku2wwckws7932qqqu8xm5ca8y",
        address2: "wasm1xv9tklw7d82sezh9haa573wufgy59vmwnxhnsl",
        address3: "wasm17yg9mssjenmc3jkqth6ulcwj9cxujrxxg9nmzk",
        address4: "wasm1f7j7ryulwjfe9ljplvhtcaxa6wqgula3nh873j"
    };
}
var aliceData = {
    mnemonic: "enlist hip relief stomach skate base shallow young switch frequent cry park",
    address0: "wasm14qemq0vw6y3gc3u3e0aty2e764u4gs5lndxgyk",
    address1: "wasm1hhg2rlu9jscacku2wwckws7932qqqu8xm5ca8y",
    address2: "wasm1xv9tklw7d82sezh9haa573wufgy59vmwnxhnsl",
    address3: "wasm17yg9mssjenmc3jkqth6ulcwj9cxujrxxg9nmzk",
    address4: "wasm1f7j7ryulwjfe9ljplvhtcaxa6wqgula3nh873j"
};
function main(wasmPath) {
    return __awaiter(this, void 0, void 0, function () {
        var gasPrice;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4, proto_signing_1.DirectSecp256k1HdWallet.fromMnemonic(aliceData.mnemonic, { prefix: "wasm" })];
                case 1:
                    alice = _a.sent();
                    gasPrice = stargate_1.GasPrice.fromString("0.025ucosm");
                    return [2];
            }
        });
    });
}
await main(wasmPath);
console.info("The show is over.");
exports.hi = 'aloha';
