//! err definition
// bussiness err 20000
use alloc::string::FromUtf8Error;
use alloc::string::{String, ToString};
use quick_protobuf::errors::Error as qp_errors;
pub enum Err {
    ProtoErrors(qp_errors),
    FromUtf8Error(FromUtf8Error),
    Null(String),
    VaildFail(i32),
    UnlockFail(i32),
    SqlExecture(String),
    CallState(String),
    AccountErrors(String),
}

impl Err {
    pub fn get(self) -> (usize, String) {
        return match self {
            Err::ProtoErrors(e) => match e {
                qp_errors::Io => (20001, "quick protobuf IO ERROR".to_string()),
                qp_errors::Utf8(err) => {
                    (20002, alloc::format!("quick protobuf Utf8 ERROR:{:?}", err))
                }
                qp_errors::Deprecated(err) => (
                    20003,
                    alloc::format!("quick protobuf Deprecated ERROR:{:?}", err),
                ),
                qp_errors::UnknownWireType(err) => (
                    20004,
                    alloc::format!("quick protobuf UnknownWireType ERROR:{:?}", err),
                ),
                qp_errors::Varint => (20004, "quick protobuf VarInt ERROR".to_string()),
                qp_errors::Map(err) => {
                    (20005, alloc::format!("quick protobuf Map ERROR:{:?}", err))
                }
                qp_errors::UnexpectedEndOfBuffer => (
                    20006,
                    "quick protobuf UnexpectedEndOfBuffer ERROR".to_string(),
                ),
                qp_errors::OutputBufferTooSmall => (
                    20006,
                    "quick protobuf OutputBufferTooSmall ERROR".to_string(),
                ),
            },
            Err::FromUtf8Error(err) => (20007, alloc::format!("from utf8 ERROR:{}", err)),
            Err::Null(err) => (20008, alloc::format!("Null ERROR:{:?}", err)),
            Err::VaildFail(err) => (20009, alloc::format!("VaildFail ERROR:{:?}", err)),
            Err::UnlockFail(err) => (20010, alloc::format!("UnlockFail ERROR:{:?}", err)),
            Err::SqlExecture(err) => (20011, alloc::format!("SqlExecture ERROR:{:?}", err)),
            Err::CallState(err) => (20012, alloc::format!("CallState ERROR:{:?}", err)),
            Err::AccountErrors(err) => (20013, alloc::format!("Account ERROR:{:?}", err)),
        };
    }
}
