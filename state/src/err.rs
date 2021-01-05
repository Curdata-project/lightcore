//! err definition
// bussiness err 20000
use alloc::string::FromUtf8Error;
use quick_protobuf::errors::Error as qp_errors;
pub enum Err {
    ProtoErrors(qp_errors),
    FromUtf8Error(FromUtf8Error),
}

impl Err {
    pub fn get(&self) -> (usize, &str) {
        return match self {
            Err::ProtoErrors(e) => match e {
                qp_errors::Io => (20001, "quick protobuf IO ERROR"),
                qp_errors::Utf8(err) => (
                    20002,
                    &alloc::format!("quick protobuf Utf8 ERROR:{:?}", err),
                ),
                qp_errors::Deprecated(err) => (
                    20003,
                    &alloc::format!("quick protobuf Deprecated ERROR:{:?}", err),
                ),
                qp_errors::UnknownWireType(err) => (
                    20004,
                    &alloc::format!("quick protobuf UnknownWireType ERROR:{:?}", err),
                ),
                qp_errors::Varint => (20004, "quick protobuf VarInt ERROR"),
                qp_errors::Map(err) => {
                    (20005, &alloc::format!("quick protobuf Map ERROR:{:?}", err))
                }
                qp_errors::UnexpectedEndOfBuffer => {
                    (20006, "quick protobuf UnexpectedEndOfBuffer ERROR")
                }
                qp_errors::OutputBufferTooSmall => {
                    (20006, "quick protobuf OutputBufferTooSmall ERROR")
                }
            },
            Err::FromUtf8Error(e) => (20007, &alloc::format!("from utf8 ERROR:{}", e)),
        };
    }
}
