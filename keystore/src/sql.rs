// pub const SELECT_ALL_FROM_KEYSTORE: &str = "select * from keystore";

pub const _CREATE_KEYSTORE_TABLE: &str = r#"
CREATE TABLE "keystore" (
    "account" blob NOT NULL,
    "encrypt_code" blob NOT NULL,
    "public_encrypt_type" blob NOT NULL,
    "seed" blob NOT NULL,
    "secret_key" blob NOT NULL,
    "public_key" blob NOT NULL,
    "cert" blob,
    "secret_encrypt_type" blob NOT NULL,
    "create_date" INTEGER NOT NULL,
    PRIMARY KEY ("account")
  )
"#;

#[macro_export]
macro_rules! get_list {
    ($item:expr,$page:expr) => {};
}
