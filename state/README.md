# State

状态管理模块

状态指的是节点所拥有的存储空间，此模块中管理当前节点知道的所有状态。

## 数据结构

```protobuf
syntax = "proto3";

// Unsigned State.
message State {
  uint64 size = 1; // Size of state.
  bytes state = 2; // State data.
  bytes owner = 3; // Owner's cert
  bytes lock = 4; // Unlock script. When state transact, you must use this script to unlock.
  bytes valid = 5; // Valid script. Use this script to valid this state.
}

// Signed State
message SignedState {
  bytes id = 1; // id = H(state)
  State state = 2; // State
  bytes witness = 3; // Witness cert.
  bytes signature = 4; // Signature for witness.
}
```

### 数据库结构

```
id:blob
state:blob
owner:blob
lock:blob
valid:blob
size:integer
is_valid:interger
```

## 访问接口

### RPC 接口

- list_states
- get_state
- valid_signed_state

### 内部接口

- add_state
- delete_state
