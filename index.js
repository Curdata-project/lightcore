const runtime = require("runtime");

const modules = {
    keystore: {
        path: 'target/wasm32-unkonwn-unknown-release/release/keystore.wasm',
        deps: [],
        expose: [
            {
                'type': 'callback',
                'name': 'list_accounts',
                'args': [
                    { 'name': 'page', 'type': 'number' },
                    { 'name': 'item', 'type': 'number' },
                    { 'name': 'order', 'type': 'number' },
                ],
                'return': {
                    'type': 'proto',
                    'proto': 'keystore/proto/keystore.proto',
                    'message': 'KeypairDisplayList'
                }
            },
            {
                'type': 'callback',
                'name': 'get_account',
                'args': [
                    { 'name': 'account', 'type': 'bytes' },
                ],
                'return': {
                    'type': 'proto',
                    'proto': 'keystore/proto/keystore.proto',
                    'message':'KeypairDisplay'
                }
            },
            {
                'type': 'callback',
                'name': 'new_account',
                'args': [],
                'return': {
                    'type': 'proto',
                    'proto': 'keystore/proto/keystore.proto',
                    'message': 'KeypairDisplay'
                }
            },
        ],
    },
}

runtime.run(modules);
