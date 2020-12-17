# Keystore

密钥管理模块

Account -> Keypair

### 数据库结构

### 支持的加密算法

- ED25519
- SM2
- AES256
- SM3

### 密钥对结构

- seed (encrypted)
- secret_key (encrypted)
- public_key
- type
- cert (Optional)

### RPC 接口

- list_accounts
- get_account
- new_account
- init_account
- import_account
- export_accounts
- sign_message
- lock_account
- unlock_account

### Actor 接口
- sign_message
- get_account
