
#[cfg(test)]
mod tests {
    use crate::module;
    use alloc::vec::Vec;

    #[test]
    fn rpc_test() {
        let kp = module::keystore::Keypair::default();

        let mut slice:Vec<u8> = Vec::new();
        quick_protobuf::serialize_into_slice(&kp,slice.as_mut_slice());
    }
}