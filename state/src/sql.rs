pub const CREATE_KEYSTORE_TABLE: &str = r#"
CREATE TABLE "state" (
    "id" blob NOT NULL,
    "state" blob NOT NULL,
    "owner" blob NOT NULL,
    "lock" blob NOT NULL,
    "valid" blob NOT NULL,
    "size" integer NOT NULL,
    "is_valid" integer NOT NULL,
    PRIMARY KEY ("account")
  )
"#;
