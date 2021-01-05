
use alloc::vec::Vec;
use alloc::borrow::Cow;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ContractRef<'a> {
    pub contract_id: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for ContractRef<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.contract_id = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ContractRef<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.contract_id == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.contract_id).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.contract_id != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.contract_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ContractList<'a> {
    pub contract_list: Vec<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for ContractList<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.contract_list.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ContractList<'a> {
    fn get_size(&self) -> usize {
        0
        + self.contract_list.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.contract_list { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}
