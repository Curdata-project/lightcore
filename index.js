const runtime = require("runtime");

const modules = [
  {
    name: "state",
    path: 'target/wasm32-unknown-unknown/release/state.wasm',
    deps: ["contract"],
  },
  {
    name: "contract",
    path: 'target/wasm32-unknown-unknown/release/contract.wasm',
    deps: [],
  },
  {
    name: "transaction",
    path: 'target/wasm32-unknown-unknown/release/transaction.wasm',
    deps: ["contract","state"],
  },
]

runtime.run(modules);