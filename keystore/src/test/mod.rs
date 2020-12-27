#[cfg(test)]
mod tests {
    use crate::proto;
    use alloc::vec::Vec;

    #[test]
    fn rpc_test() {
        let kp = proto::keystore::Keypair::default();

        let mut slice: Vec<u8> = Vec::new();
        //序列化
        let _result = quick_protobuf::serialize_into_slice(&kp, slice.as_mut_slice());
        let mut v: Vec<u8> = Vec::new();
        v.push(0);
        let s = v.as_slice();
        //反序列化
        let _kp = quick_protobuf::deserialize_from_slice::<proto::keystore::Keypair>(s);
    }
}
