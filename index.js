const runtime = require("runtime");

const modules = {
  keystore: {
  path: 'target/wasm32-unknown-unknown/release/keystore.wasm',
  deps: [],
    expose: [
      {
        type: 'callback',
        name: 'list_accounts',
        args: [{ name: 'page', type: 'number' },
          { name: 'item', type: 'number' },
          { name: 'order', type: 'number' },
        ],
        return: {
          type: 'proto',
          proto: 'keystore/proto/keystore.proto',
          message: 'KeypairDisplayList'
        }
      },
      {
        type: 'callback',
        name: 'get_account',
        args: [{ name: 'account', type: 'bytes' }],
        return: {
          type: 'proto',
          proto: 'keystore/proto/keystore.proto',
          message: 'KeypairDisplay'
        }
      },
      {
        type: 'callback',
        name: 'import_account',
        args: [
          {
            name: 'keypair',
            type: 'proto',
            attr: {
              proto: 'keystore/proto/keystore.proto',
              message: 'Keypair'
            }
          }
        ],
        return: {
          type: 'number',
        }
      },
      {
        type: 'callback',
        name: 'new_account',
        args: [{name: 'AccountMsg', type: 'proto'}],
        return: {
            type: 'proto',
            proto: 'keystore/proto/keystore.proto',
            message: 'KeypairDisplay'
        }
      },
    ],
  },
}

runtime.run(modules);
