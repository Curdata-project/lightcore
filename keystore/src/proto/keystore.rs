use super::*;
use alloc::borrow::Cow;
use alloc::vec::Vec;
use quick_protobuf::sizeofs::*;
use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Result, Writer, WriterBackend};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Keypair<'a> {
    pub account: Cow<'a, [u8]>,
    pub seed: Cow<'a, [u8]>,
    pub secret_key: Cow<'a, [u8]>,
    pub public_key: Cow<'a, [u8]>,
    pub type_pb: Cow<'a, str>,
    pub cert: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for Keypair<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.account = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.seed = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.secret_key = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.public_key = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.type_pb = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.cert = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Keypair<'a> {
    fn get_size(&self) -> usize {
        0 + if self.account == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.account).len())
        } + if self.seed == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.seed).len())
        } + if self.secret_key == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.secret_key).len())
        } + if self.public_key == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.public_key).len())
        } + if self.type_pb == "" {
            0
        } else {
            1 + sizeof_len((&self.type_pb).len())
        } + if self.cert == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.cert).len())
        }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.account != Cow::Borrowed(b"") {
            w.write_with_tag(10, |w| w.write_bytes(&**&self.account))?;
        }
        if self.seed != Cow::Borrowed(b"") {
            w.write_with_tag(18, |w| w.write_bytes(&**&self.seed))?;
        }
        if self.secret_key != Cow::Borrowed(b"") {
            w.write_with_tag(26, |w| w.write_bytes(&**&self.secret_key))?;
        }
        if self.public_key != Cow::Borrowed(b"") {
            w.write_with_tag(34, |w| w.write_bytes(&**&self.public_key))?;
        }
        if self.type_pb != "" {
            w.write_with_tag(42, |w| w.write_string(&**&self.type_pb))?;
        }
        if self.cert != Cow::Borrowed(b"") {
            w.write_with_tag(50, |w| w.write_bytes(&**&self.cert))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct KeypairDisplay<'a> {
    pub account: Cow<'a, [u8]>,
    pub public_key: Cow<'a, [u8]>,
    pub type_pb: Cow<'a, str>,
    pub cert: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for KeypairDisplay<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.account = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.public_key = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.type_pb = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.cert = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for KeypairDisplay<'a> {
    fn get_size(&self) -> usize {
        0 + if self.account == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.account).len())
        } + if self.public_key == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.public_key).len())
        } + if self.type_pb == "" {
            0
        } else {
            1 + sizeof_len((&self.type_pb).len())
        } + if self.cert == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.cert).len())
        }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.account != Cow::Borrowed(b"") {
            w.write_with_tag(10, |w| w.write_bytes(&**&self.account))?;
        }
        if self.public_key != Cow::Borrowed(b"") {
            w.write_with_tag(34, |w| w.write_bytes(&**&self.public_key))?;
        }
        if self.type_pb != "" {
            w.write_with_tag(42, |w| w.write_string(&**&self.type_pb))?;
        }
        if self.cert != Cow::Borrowed(b"") {
            w.write_with_tag(50, |w| w.write_bytes(&**&self.cert))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct KeypairDisplayList<'a> {
    pub keypair_display_list: Vec<KeypairDisplay<'a>>,
}

impl<'a> MessageRead<'a> for KeypairDisplayList<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg
                    .keypair_display_list
                    .push(r.read_message::<KeypairDisplay>(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for KeypairDisplayList<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .keypair_display_list
            .iter()
            .map(|s| 1 + sizeof_len((s).get_size()))
            .sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.keypair_display_list {
            w.write_with_tag(10, |w| w.write_message(s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Keystore<'a> {
    pub account: Cow<'a, [u8]>,
    pub encrypt_code: Cow<'a, [u8]>,
    pub public_encrypt_type: Cow<'a, str>,
    pub secret_encrypt_type: Cow<'a, str>,
    pub public_key: Cow<'a, [u8]>,
    pub secret_key: Cow<'a, [u8]>,
    pub cert: Cow<'a, [u8]>,
    pub create_date: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Keystore<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.account = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.encrypt_code = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.public_encrypt_type = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.secret_encrypt_type = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.public_key = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.secret_key = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.cert = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(66) => msg.create_date = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Keystore<'a> {
    fn get_size(&self) -> usize {
        0 + if self.account == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.account).len())
        } + if self.encrypt_code == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.encrypt_code).len())
        } + if self.public_encrypt_type == "" {
            0
        } else {
            1 + sizeof_len((&self.public_encrypt_type).len())
        } + if self.secret_encrypt_type == "" {
            0
        } else {
            1 + sizeof_len((&self.secret_encrypt_type).len())
        } + if self.public_key == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.public_key).len())
        } + if self.secret_key == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.secret_key).len())
        } + if self.cert == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.cert).len())
        } + if self.create_date == "" {
            0
        } else {
            1 + sizeof_len((&self.create_date).len())
        }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.account != Cow::Borrowed(b"") {
            w.write_with_tag(10, |w| w.write_bytes(&**&self.account))?;
        }
        if self.encrypt_code != Cow::Borrowed(b"") {
            w.write_with_tag(18, |w| w.write_bytes(&**&self.encrypt_code))?;
        }
        if self.public_encrypt_type != "" {
            w.write_with_tag(26, |w| w.write_string(&**&self.public_encrypt_type))?;
        }
        if self.secret_encrypt_type != "" {
            w.write_with_tag(34, |w| w.write_string(&**&self.secret_encrypt_type))?;
        }
        if self.public_key != Cow::Borrowed(b"") {
            w.write_with_tag(42, |w| w.write_bytes(&**&self.public_key))?;
        }
        if self.secret_key != Cow::Borrowed(b"") {
            w.write_with_tag(50, |w| w.write_bytes(&**&self.secret_key))?;
        }
        if self.cert != Cow::Borrowed(b"") {
            w.write_with_tag(58, |w| w.write_bytes(&**&self.cert))?;
        }
        if self.create_date != "" {
            w.write_with_tag(66, |w| w.write_string(&**&self.create_date))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct KeystoreList<'a> {
    pub keystore_list: Vec<Keypair<'a>>,
}

impl<'a> MessageRead<'a> for KeystoreList<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.keystore_list.push(r.read_message::<Keypair>(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for KeystoreList<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .keystore_list
            .iter()
            .map(|s| 1 + sizeof_len((s).get_size()))
            .sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.keystore_list {
            w.write_with_tag(10, |w| w.write_message(s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Sign<'a> {
    pub account: Cow<'a, [u8]>,
    pub message: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Sign<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.account = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.message = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Sign<'a> {
    fn get_size(&self) -> usize {
        0 + if self.account == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.account).len())
        } + if self.message == "" {
            0
        } else {
            1 + sizeof_len((&self.message).len())
        }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.account != Cow::Borrowed(b"") {
            w.write_with_tag(10, |w| w.write_bytes(&**&self.account))?;
        }
        if self.message != "" {
            w.write_with_tag(18, |w| w.write_string(&**&self.message))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct OptionLock<'a> {
    pub account: Cow<'a, [u8]>,
    pub encrypt_code: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for OptionLock<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.account = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.encrypt_code = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for OptionLock<'a> {
    fn get_size(&self) -> usize {
        0 + if self.account == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.account).len())
        } + if self.encrypt_code == "" {
            0
        } else {
            1 + sizeof_len((&self.encrypt_code).len())
        }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.account != Cow::Borrowed(b"") {
            w.write_with_tag(10, |w| w.write_bytes(&**&self.account))?;
        }
        if self.encrypt_code != "" {
            w.write_with_tag(18, |w| w.write_string(&**&self.encrypt_code))?;
        }
        Ok(())
    }
}
