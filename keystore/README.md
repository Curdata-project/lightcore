# Keystore

密钥管理模块

Account -> Keypair

### 数据库结构

- 表字段
  - account 账户
  - encrypt_code 加密码，用户给/随机生成，用来加密公钥和seed
  - public_encrypt_type 公钥+种子 加密类型
  - seed
  - secret_key
  - public_key
  - cert
  - secret_encrypt_type 私钥 加密类型
  - create_date 创建时间
  
### proto

```protobuf
syntax = "proto3";

// 密钥对数据格式
message Keypair {
  string account = 1;
  string seed = 2;
  string secret_key = 3;
  string public_key = 4;
  string type = 5;
  string cert = 6;
}

// 密钥对list数据格式
message KeyPairList {
  repeated Keypair keypair_list = 1;
}

// 数据库对应字段
message Keystore{
  string account = 1; // 账户
  string encrypt_code = 2; // 加密码，用户给/随机生成，用来加密公钥和seed
  string public_encrypt_type = 3; // 公钥+种子 加密类型
  string secret_encrypt_type = 4; // 私钥 加密类型
  string public_key = 5;
  string secret_key = 6;
  string cert = 7;
  string create_date = 8;
}

// 签名结构体
message Sign{
  string message = 1; //需签名的数据
  string type = 2; //签名类型
}

// 解锁/上锁
message OptionLock{
  string account = 1;
  string encrypt_code = 2; // 解锁/上锁码
}

// 统一请求格式
message Request{
  oneof data{
    string account = 1;
    OptionLock option_lock = 2;
    Sign sign = 3;
  }
}

// 统一返回格式
message Response{
  oneof data {
    Keypair keypair = 2;
    Keystore keystore = 3;
    KeyPairList keypair_list = 4;
    uint32 code = 5;
  }
}

```

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

- #### list_accounts
  - 参数：page usize,item usize,order usize
  - 返回值：[u8]
    
- #### get_account
  - 参数：ptr *mut usize,size usize
  - 返回值：[u8]
  
- #### new_account 随机生成密码
  - 参数：ptr *mut usize,size usize
  - 返回值：[u8]
  
- #### import_account
  - 参数：ptr *mut usize,size usize
  - 返回值：[u8]
  
- #### export_accounts
  - 参数：ptr *mut usize,size usize
  - 返回值：[u8]
  
- #### sign_message
  - 参数：ptr *mut usize,size usize
  - 返回值：[u8]
    
- #### lock_account
  - 参数：ptr *mut usize,size usize
  - 返回值：[u8]
  
- #### unlock_account
  - 参数：ptr *mut usize,size usize
  - 返回值：[u8]

### Actor 接口

- #### sign_message
  - 参数：ptr *mut usize,size usize
  - 返回值：[u8]
    
- #### get_account
  - 参数：ptr *mut usize,size usize
  - 返回值：[u8]