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
  bytes account = 1; //账户
  bytes seed = 2; // 种子
  bytes secret_key = 3; // 密钥
  bytes public_key = 4; // 公钥
  string type = 5; // 生成密钥的算法类型
  bytes cert = 6; // 证书
}

// 密钥对list数据格式
message KeyPairList {
  repeated Keypair keypair_list = 1; // 密钥对数组
}

// 数据库对应字段
message Keystore{
  bytes account = 1; // 账户
  bytes encrypt_code = 2; // 加密码，用户给/随机生成，用来加密公钥和seed
  string public_encrypt_type = 3; // 公钥+种子 加密类型
  string secret_encrypt_type = 4; // 私钥 加密类型
  bytes public_key = 5; // 公钥
  bytes secret_key = 6; // 私钥
  bytes cert = 7; // 证书
   create_date = 8; // 生成时间
}

//数据库数据list
message KeystoreList{
  repeated Keypair keystore_list = 1;
}

// 签名结构体
message Sign{
  bytes account = 1; //账户
  string message = 2; //需签名的数据
}

// 解锁/上锁
message OptionLock{
  bytes account = 1;
  string encrypt_code = 2; // 解锁/上锁码
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

- #### _list_accounts 
  - 参数：page number,item number,order number
    - page: 页数
    - item: 每页条数
    - order: 0 正序 1倒序
    - index: 下标
      - return:KeystoreList proto bytes
    
- #### _get_account
  - 参数：ptr number,size number
    - ptr: 指针
    - size: 长度
      - return:KeypairDisplay proto bytes
  
- #### _new_account 生成账户
    - 参数：index number
      - index：下标
      - return:KeypairDisplay proto bytes
  
- #### _import_account 导入账户
  - 参数：ptr *mut number,size number
    - ptr:无符号整数指针
    - size:数据的长度
    - index：下标
      - return:usize 
  
- #### _export_accounts 导出账户
  - 参数：ptr *mut number,size number
    - ptr:无符号整数指针
    - size:数据的长度
    - index：下标
      - return:Keypair proto bytes
  
- #### _sign_message 签名
  - 参数：ptr *mut number,size number
    - ptr:无符号整数指针
    - size:数据的长度
    - index：下标
      - return:usize 
    
- #### _lock_account 加锁
  - 参数：ptr *mut number,size number
    - ptr:无符号整数指针
    - size:数据的长度
    - index：下标
      - return:usize 
  
- #### _unlock_account 解锁
  - 参数：ptr *mut number,size number
    - ptr:无符号整数指针
    - size:数据的长度
    - index：下标
      - return:usize 

### Actor 接口

- #### _sign_message
  - 参数：ptr *mut number,size number
    - ptr:无符号整数指针
    - size:数据的长度
    - index：下标
      - return:usize 
    
- #### _get_account
  - 参数：ptr *mut number,size number
    - ptr:无符号整数指针
    - size:数据的长度
      - return:Keypair proto bytes