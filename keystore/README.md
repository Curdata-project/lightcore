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
  - nonce 随机数
  
### proto

```protobuf
syntax = "proto3";

// 对外rpc参数-------------------------------------------------------------------

// 密钥对数据格式
message Keypair {
  bytes account = 1; //账户
  bytes seed = 2; // 种子
  bytes secret_key = 3; // 密钥
  bytes public_key = 4; // 公钥
  string ty = 5; // 生成密钥的算法类型
  bytes cert = 6; // 证书
  bytes encrypt_code = 7; // 加密码
  bytes nonce = 8; // 随机数
}

// 密钥对展示格式
message KeypairDisplay{
  bytes account = 1; //账户
  bytes public_key = 4; // 公钥
  string ty = 5; // 生成密钥的算法类型
  bytes cert = 6; // 证书
}

// 密钥对list数据格式
message KeypairDisplayList {
  repeated KeypairDisplay keypair_display_list = 1; // 密钥对数组
}

// 账户信息
message AccountMsg{
  bytes account =1; //账户
  bytes encrypt_code = 2; //对称加密key
}

message Sign {
  AccountMsg account_msg = 1; //账户信息
  bytes message = 2;
}

// 公共签名验证
message PubVerifySign{
  bytes public_key = 1; //公钥
  bytes sign = 2; //签名
  bytes message = 3; //需验证的信息
}

// 账户签名验证
message AccountVerifySign{
  AccountMsg account_msg = 1; //账户信息
  bytes message = 2;// 需验证信息
  bytes sign = 3; // 签名
}

// 签名体
message VerifySign{
  oneof VerfySign{
    AccountVerifySign AccountVerifySign = 1; //账户签名验证
    PubVerifySign PubVerifySign = 2; //公共签名验证
  }
}


// 对内rpc参数----------------------------------------------------------------------

// 数据库对应字段
message Keystore{
  bytes account = 1; // 账户
  bytes seed = 2; // 种子
  bytes encrypt_code = 3; // 加密码，用户给/随机生成，用来加密公钥和seed
  string public_encrypt_type = 4; // 公钥+种子 加密类型
  string secret_encrypt_type = 5; // 私钥 加密类型
  bytes public_key = 6; // 公钥
  bytes secret_key = 7; // 私钥
  bytes cert = 8; // 证书
  int64 timestamp = 9; // 生成时间
  bytes nonce = 10; //随机数
}

//数据库数据list
message KeystoreList{
  repeated Keypair keystore_list = 1;
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
  - 参数：page number,item number,order number
    - page: 页数
    - item: 每页条数
    - order: 0 正序 1倒序
    - index: 下标
      - return:KeystoreList proto bytes
    
- #### get_account
  - 参数：ptr number,size number
    - ptr: 指针
    - size: 长度
      - return:KeypairDisplay proto bytes
  
- #### new_account 生成账户
    - 参数：index number
      - index：下标
      - return:KeypairDisplay proto bytes
  
- #### import_account 导入账户
  - 参数：ptr *mut number,size number
    - ptr:无符号整数指针
    - size:数据的长度
    - index：下标
      - return:usize 
  
- #### export_accounts 导出账户
  - 参数：ptr *mut number,size number
    - ptr:无符号整数指针
    - size:数据的长度
    - index：下标
      - return:Keypair proto bytes
  
- #### sign_message 签名
  - 参数：ptr *mut number,size number
    - ptr:无符号整数指针
    - size:数据的长度
    - index：下标
      - return:usize 
    
- #### lock_account 加锁
  - 参数：ptr *mut number,size number
    - ptr:无符号整数指针
    - size:数据的长度
    - index：下标
      - return:usize 
  
- #### unlock_account 解锁
  - 参数：ptr *mut number,size number
    - ptr:无符号整数指针
    - size:数据的长度
    - index：下标
      - return:usize 

### Actor 接口

- #### sign_message
  - 参数：ptr *mut number,size number
    - ptr:无符号整数指针
    - size:数据的长度
    - index：下标
      - return:usize 
    
- #### get_account
  - 参数：ptr *mut number,size number
    - ptr:无符号整数指针
    - size:数据的长度
      - return:Keypair proto bytes