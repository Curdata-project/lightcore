
use alloc::vec::Vec;
use alloc::borrow::Cow;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct State<'a> {
    pub size: u64,
    pub state: Cow<'a, [u8]>,
    pub owner: Cow<'a, [u8]>,
    pub lock: Cow<'a, [u8]>,
    pub valid: Cow<'a, [u8]>,
    pub msg: Option<common::Msg<'a>>,
}

impl<'a> MessageRead<'a> for State<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.size = r.read_uint64(bytes)?,
                Ok(18) => msg.state = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.owner = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.lock = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.valid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.msg = Some(r.read_message::<common::Msg>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for State<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.size == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.size) as u64) }
        + if self.state == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.state).len()) }
        + if self.owner == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.owner).len()) }
        + if self.lock == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.lock).len()) }
        + if self.valid == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.valid).len()) }
        + self.msg.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.size != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.size))?; }
        if self.state != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.state))?; }
        if self.owner != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.owner))?; }
        if self.lock != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.lock))?; }
        if self.valid != Cow::Borrowed(b"") { w.write_with_tag(42, |w| w.write_bytes(&**&self.valid))?; }
        if let Some(ref s) = self.msg { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SignedState<'a> {
    pub id: Cow<'a, [u8]>,
    pub state: Option<State<'a>>,
    pub witness: Cow<'a, [u8]>,
    pub signature: Cow<'a, [u8]>,
    pub msg: Option<common::Msg<'a>>,
}

impl<'a> MessageRead<'a> for SignedState<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.state = Some(r.read_message::<State>(bytes)?),
                Ok(26) => msg.witness = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.signature = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.msg = Some(r.read_message::<common::Msg>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SignedState<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.state.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.witness == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.witness).len()) }
        + if self.signature == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.signature).len()) }
        + self.msg.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.id))?; }
        if let Some(ref s) = self.state { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.witness != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.witness))?; }
        if self.signature != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.signature))?; }
        if let Some(ref s) = self.msg { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

