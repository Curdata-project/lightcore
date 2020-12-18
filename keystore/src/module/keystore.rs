// Automatically generated rust module for 'keystore.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use alloc::vec::Vec;
use alloc::borrow::Cow;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Keypair<'a> {
    pub account: Cow<'a, str>,
    pub seed: Cow<'a, str>,
    pub secret_key: Cow<'a, str>,
    pub public_key: Cow<'a, str>,
    pub type_pb: Cow<'a, str>,
    pub cert: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Keypair<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.account = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.seed = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.secret_key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.public_key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.type_pb = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.cert = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Keypair<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.account == "" { 0 } else { 1 + sizeof_len((&self.account).len()) }
        + if self.seed == "" { 0 } else { 1 + sizeof_len((&self.seed).len()) }
        + if self.secret_key == "" { 0 } else { 1 + sizeof_len((&self.secret_key).len()) }
        + if self.public_key == "" { 0 } else { 1 + sizeof_len((&self.public_key).len()) }
        + if self.type_pb == "" { 0 } else { 1 + sizeof_len((&self.type_pb).len()) }
        + if self.cert == "" { 0 } else { 1 + sizeof_len((&self.cert).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.account != "" { w.write_with_tag(10, |w| w.write_string(&**&self.account))?; }
        if self.seed != "" { w.write_with_tag(18, |w| w.write_string(&**&self.seed))?; }
        if self.secret_key != "" { w.write_with_tag(26, |w| w.write_string(&**&self.secret_key))?; }
        if self.public_key != "" { w.write_with_tag(34, |w| w.write_string(&**&self.public_key))?; }
        if self.type_pb != "" { w.write_with_tag(42, |w| w.write_string(&**&self.type_pb))?; }
        if self.cert != "" { w.write_with_tag(50, |w| w.write_string(&**&self.cert))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct KeyPairList<'a> {
    pub keypair_list: Vec<Keypair<'a>>,
}

impl<'a> MessageRead<'a> for KeyPairList<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.keypair_list.push(r.read_message::<Keypair>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for KeyPairList<'a> {
    fn get_size(&self) -> usize {
        0
        + self.keypair_list.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.keypair_list { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Keystore<'a> {
    pub account: Cow<'a, str>,
    pub encrypt_code: Cow<'a, str>,
    pub public_encrypt_type: Cow<'a, str>,
    pub secret_encrypt_type: Cow<'a, str>,
    pub public_key: Cow<'a, str>,
    pub secret_key: Cow<'a, str>,
    pub cert: Cow<'a, str>,
    pub create_date: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Keystore<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.account = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.encrypt_code = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.public_encrypt_type = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.secret_encrypt_type = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.public_key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.secret_key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.cert = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(66) => msg.create_date = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Keystore<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.account == "" { 0 } else { 1 + sizeof_len((&self.account).len()) }
        + if self.encrypt_code == "" { 0 } else { 1 + sizeof_len((&self.encrypt_code).len()) }
        + if self.public_encrypt_type == "" { 0 } else { 1 + sizeof_len((&self.public_encrypt_type).len()) }
        + if self.secret_encrypt_type == "" { 0 } else { 1 + sizeof_len((&self.secret_encrypt_type).len()) }
        + if self.public_key == "" { 0 } else { 1 + sizeof_len((&self.public_key).len()) }
        + if self.secret_key == "" { 0 } else { 1 + sizeof_len((&self.secret_key).len()) }
        + if self.cert == "" { 0 } else { 1 + sizeof_len((&self.cert).len()) }
        + if self.create_date == "" { 0 } else { 1 + sizeof_len((&self.create_date).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.account != "" { w.write_with_tag(10, |w| w.write_string(&**&self.account))?; }
        if self.encrypt_code != "" { w.write_with_tag(18, |w| w.write_string(&**&self.encrypt_code))?; }
        if self.public_encrypt_type != "" { w.write_with_tag(26, |w| w.write_string(&**&self.public_encrypt_type))?; }
        if self.secret_encrypt_type != "" { w.write_with_tag(34, |w| w.write_string(&**&self.secret_encrypt_type))?; }
        if self.public_key != "" { w.write_with_tag(42, |w| w.write_string(&**&self.public_key))?; }
        if self.secret_key != "" { w.write_with_tag(50, |w| w.write_string(&**&self.secret_key))?; }
        if self.cert != "" { w.write_with_tag(58, |w| w.write_string(&**&self.cert))?; }
        if self.create_date != "" { w.write_with_tag(66, |w| w.write_string(&**&self.create_date))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response<'a> {
    pub code: u32,
    pub data: mod_Response::OneOfdata<'a>,
}

impl<'a> MessageRead<'a> for Response<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.code = r.read_uint32(bytes)?,
                Ok(18) => msg.data = mod_Response::OneOfdata::keypair(r.read_message::<Keypair>(bytes)?),
                Ok(26) => msg.data = mod_Response::OneOfdata::keystore(r.read_message::<Keystore>(bytes)?),
                Ok(34) => msg.data = mod_Response::OneOfdata::keypair_list(r.read_message::<KeyPairList>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Response<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.code == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.code) as u64) }
        + match self.data {
            mod_Response::OneOfdata::keypair(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfdata::keystore(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfdata::keypair_list(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfdata::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.code != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.code))?; }
        match self.data {            mod_Response::OneOfdata::keypair(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_Response::OneOfdata::keystore(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_Response::OneOfdata::keypair_list(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_Response::OneOfdata::None => {},
    }        Ok(())
    }
}

pub mod mod_Response {

use alloc::vec::Vec;
use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfdata<'a> {
    keypair(Keypair<'a>),
    keystore(Keystore<'a>),
    keypair_list(KeyPairList<'a>),
    None,
}

impl<'a> Default for OneOfdata<'a> {
    fn default() -> Self {
        OneOfdata::None
    }
}

}

