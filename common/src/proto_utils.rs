use alloc::vec;
use alloc::vec::Vec;
use quick_protobuf::{message::MessageWrite, sizeofs, writer, Result};

pub fn qb_serialize<M: MessageWrite>(message: &M) -> Result<Vec<u8>> {
    let out_len = sizeofs::sizeof_len(message.get_size());
    let mut out = vec![0u8; out_len];

    let result = writer::serialize_into_slice(message, out.as_mut_slice());
    return match result {
        Ok(_v) => Ok(out.to_vec()),
        Err(e) => Err(e),
    };
}
