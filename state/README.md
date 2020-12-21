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
  bytes script = 4; // Smart contract support.
  bytes witness = 5; // Witness's cert.
}

// Signed State
message SignedState {
  bytes id = 1; // id = H(state)
  State state = 2; // State
  bytes signature = 3; // signature = Sign(state, witeness's sk), notice: this field maybe remove later.
}

```

### 数据库结构

TODO

## 访问接口

### RPC 接口

- list_states
- get_state
- valid_signed_state

### 内部接口

- add_state
- delete_state
