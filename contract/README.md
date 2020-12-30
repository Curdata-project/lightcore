# Contract

智能合约模块

智能合约模块负责执行状态中的合约，实现对状态和交易的正确性验证。

## 数据结构

```protobuf
syntax = "proto3";

// Input of transaction.
message ContractRef {
  bytes stateid = 1; // Store contract bytecode in state. use this field to reference state.
}

```

### 数据库结构

TODO:

### RPC接口

- load_contract
- get_contract
  - true/false
- list_contracts

### 内部接口

- run_contract
