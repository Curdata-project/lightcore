
use alloc::vec::Vec;
use alloc::borrow::Cow;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Sql<'a> {
    pub sql: Cow<'a, str>,
    pub params: Vec<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for Sql<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.sql = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.params.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Sql<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.sql == "" { 0 } else { 1 + sizeof_len((&self.sql).len()) }
        + self.params.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.sql != "" { w.write_with_tag(10, |w| w.write_string(&**&self.sql))?; }
        for s in &self.params { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Msg<'a> {
    pub code: i32,
    pub detail: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Msg<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.code = r.read_int32(bytes)?,
                Ok(18) => msg.detail = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Msg<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.code == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.code) as u64) }
        + if self.detail == "" { 0 } else { 1 + sizeof_len((&self.detail).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.code != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.code))?; }
        if self.detail != "" { w.write_with_tag(18, |w| w.write_string(&**&self.detail))?; }
        Ok(())
    }
}

