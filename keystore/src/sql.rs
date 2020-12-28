// pub const SELECT_ALL_FROM_KEYSTORE: &str = "select * from keystore";

pub const CREATE_KEYSTORE_TABLE: &str = r#"
CREATE TABLE "keystore_db" (
    "account" VARCHAR(255) NOT NULL,
    "encrypt_code" VARCHAR(255) NOT NULL,
    "public_encrypt_type" VARCHAR(255) NOT NULL,
    "seed" VARCHAR(255) NOT NULL,
    "secret_key" VARCHAR(255) NOT NULL,
    "public_key" VARCHAR(255) NOT NULL,
    "cert" VARCHAR(255),
    "secret_encrypt_type" VARCHAR(255) NOT NULL,
    "create_date" INTEGER NOT NULL,
    PRIMARY KEY ("account")
  )
"#;

#[macro_export]
macro_rules! get_list {
    ($item:expr,$page:expr) => {};
}
