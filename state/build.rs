extern crate pb_rs;

use pb_rs::types::{Config, FileDescriptor, RpcService};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::vec::Vec;

fn main() {
    fn generate_rpc_test<W: Write + ?Sized>(
        rpc: &RpcService,
        w: &mut W,
    ) -> Result<(), pb_rs::errors::Error> {
        /* Example:
            trait <service> {
                fn <func>(&self, arg: &<arg>) -> Result<<ret>, failure::Error>;
            }
        */

        writeln!(w, "\npub trait {SERVICE} {{", SERVICE = rpc.service_name)?;
        for func in rpc.functions.iter() {
            writeln!(
                w,
                "   fn {FUNC}(&self, arg: &{ARG}) -> std::result::Result<{RET}, quick_protobuf::Error>;",
                FUNC = func.name,
                ARG = func.arg,
                RET = func.ret
            )?;
        }
        writeln!(w, "}}\n")?;

        Ok(())
    }

    let quick_dest = Path::new("./src/proto");

    let common_config = Config {
        in_file: PathBuf::from("./proto/common.proto"),
        out_file: quick_dest.join("common.rs"),
        single_module: true,
        import_search_path: vec![PathBuf::from("../proto")],
        no_output: false,
        error_cycle: false,
        headers: false,
        dont_use_cow: false,
        custom_struct_derive: vec![],
        custom_repr: None,
        custom_rpc_generator: Box::new(|rpc, writer| generate_rpc_test(rpc, writer)),
        custom_includes: Vec::new(),
        owned: false,
        hashbrown: false,
        nostd: true,
    };

    let state_config = Config {
        in_file: PathBuf::from("./proto/state.proto"),
        out_file: quick_dest.join("state.rs"),
        single_module: true,
        import_search_path: vec![PathBuf::from("../proto")],
        no_output: false,
        error_cycle: false,
        headers: false,
        dont_use_cow: false,
        custom_struct_derive: vec![],
        custom_repr: None,
        custom_rpc_generator: Box::new(|rpc, writer| generate_rpc_test(rpc, writer)),
        custom_includes: Vec::new(),
        owned: false,
        hashbrown: false,
        nostd: true,
    };

    let mut v: Vec<Config> = Vec::new();
    v.push(state_config);
    v.push(common_config);

    FileDescriptor::run(&v).unwrap();
}
