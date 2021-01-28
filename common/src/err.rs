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
    InitErrors(String),
    Transaction(String),
}

impl Err {
    pub fn get(self) -> (usize, String) {
        return match self {
            Err::ProtoErrors(e) => match e {
                qp_errors::Io => (20001, "20001:IO:quick protobuf IO ERROR".to_string()),
                qp_errors::Utf8(err) => (20002, alloc::format!("20002:Utf8:{:?}", err)),
                qp_errors::Deprecated(err) => (20003, alloc::format!("20003:Deprecated:{:?}", err)),
                qp_errors::UnknownWireType(err) => {
                    (20004, alloc::format!("20004:UnknownWireType:{:?}", err))
                }
                qp_errors::Varint => (
                    20005,
                    "20005:Varint:quick protobuf VarInt ERROR".to_string(),
                ),
                qp_errors::Map(err) => (20006, alloc::format!("20006:Map:{:?}", err)),
                qp_errors::UnexpectedEndOfBuffer => (
                    20007,
                    "20007:UnexpectedEndOfBuffer:quick protobuf UnexpectedEndOfBuffer ERROR"
                        .to_string(),
                ),
                qp_errors::OutputBufferTooSmall => (
                    20008,
                    "20008:OutputBufferTooSmall:quick protobuf OutputBufferTooSmall ERROR"
                        .to_string(),
                ),
            },
            Err::FromUtf8Error(err) => (20009, alloc::format!("20009:FromUtf8Error:{:?}", err)),
            Err::Null(err) => (20010, alloc::format!("20010:Null:{:?}", err)),
            Err::VaildFail(err) => (20011, alloc::format!("20011:VaildFail:{:?}", err)),
            Err::UnlockFail(err) => (20012, alloc::format!("20012:UnlockFail:{:?}", err)),
            Err::SqlExecture(err) => (20013, alloc::format!("20013:SqlExecture:{:?}", err)),
            Err::CallState(err) => (20014, alloc::format!("20014:CallState:{:?}", err)),
            Err::AccountErrors(err) => (20015, alloc::format!("20015:AccountErrors:{:?}", err)),
            Err::InitErrors(err) => (20016, alloc::format!("20016:Initerrors:{:?}", err)),
            Err::Transaction(err) => (20017, alloc::format!("20017:Transaction:{:?}",err)),
        };
    }
}
