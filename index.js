const runtime = require("runtime");

const modules = {
    keystore: {
        path: 'target/wasm32-unkonwn-unknown-release/release/keystore.wasm',
        deps: [],
        // expose: [
        //     'get_account',
        // ],
    },
    // transcation: {
    //     path: 'target/wasm32-unkonwn-unknown-release/release/transcation.wasm',
    //     deps: ['keystore'],
    // }
}

runtime.load(modules);
runtime.run();
