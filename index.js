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
                    {
                        'name': 'page',
                        'type': 'number',
                        'attr': {
                            'proto': '',
                            'message': '',
                        }
                    },
                    {
                        'name': 'item',
                        'type': 'number',
                        'attr': {
                            'proto': '',
                            'message': '',
                        }
                    },
                    {
                        'name': 'order',
                        'type': 'number',
                        'attr': {
                            'proto': '',
                            'message': '',
                        }
                    },
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
                    {
                        'name': 'account',
                        'type': 'bytes',
                        'attr': {
                            'proto': '',
                            'message': '',
                        }
                    },
                ],
                'return': {
                    'type': 'proto',
                    'proto': 'keystore/proto/keystore.proto',
                    'message': 'KeypairDisplay'
                }
            },
            {
                'type': 'callback',
                'name': 'import_account',
                'args': [
                    {
                        'name': 'keypair',
                        'type': 'proto',
                        'attr': {
                            'proto': 'keystore/proto/keystore.proto',
                            'message': 'Keypair.proto'
                        }
                    }
                ],
                'return': {
                    'type': 'number',
                    'proto': '',
                    'message': ''
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
