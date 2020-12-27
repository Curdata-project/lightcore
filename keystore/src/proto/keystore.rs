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
    pub ty: Cow<'a, str>,
    pub cert: Cow<'a, [u8]>,
    pub encrypt_code: Cow<'a, [u8]>,
    pub nonce: Cow<'a, [u8]>,
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
                Ok(42) => msg.ty = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.cert = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.encrypt_code = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(66) => msg.nonce = r.read_bytes(bytes).map(Cow::Borrowed)?,
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
        } + if self.ty == "" {
            0
        } else {
            1 + sizeof_len((&self.ty).len())
        } + if self.cert == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.cert).len())
        } + if self.encrypt_code == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.encrypt_code).len())
        } + if self.nonce == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.nonce).len())
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
        if self.ty != "" {
            w.write_with_tag(42, |w| w.write_string(&**&self.ty))?;
        }
        if self.cert != Cow::Borrowed(b"") {
            w.write_with_tag(50, |w| w.write_bytes(&**&self.cert))?;
        }
        if self.encrypt_code != Cow::Borrowed(b"") {
            w.write_with_tag(58, |w| w.write_bytes(&**&self.encrypt_code))?;
        }
        if self.nonce != Cow::Borrowed(b"") {
            w.write_with_tag(66, |w| w.write_bytes(&**&self.nonce))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct KeypairDisplay<'a> {
    pub account: Cow<'a, [u8]>,
    pub public_key: Cow<'a, [u8]>,
    pub ty: Cow<'a, str>,
    pub cert: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for KeypairDisplay<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.account = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.public_key = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.ty = r.read_string(bytes).map(Cow::Borrowed)?,
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
        } + if self.ty == "" {
            0
        } else {
            1 + sizeof_len((&self.ty).len())
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
        if self.ty != "" {
            w.write_with_tag(42, |w| w.write_string(&**&self.ty))?;
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
    pub seed: Cow<'a, [u8]>,
    pub encrypt_code: Cow<'a, [u8]>,
    pub public_encrypt_type: Cow<'a, str>,
    pub secret_encrypt_type: Cow<'a, str>,
    pub public_key: Cow<'a, [u8]>,
    pub secret_key: Cow<'a, [u8]>,
    pub cert: Cow<'a, [u8]>,
    pub timestamp: i64,
    pub nonce: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for Keystore<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.account = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.seed = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.encrypt_code = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.public_encrypt_type = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.secret_encrypt_type = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.public_key = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.secret_key = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(66) => msg.cert = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(72) => msg.timestamp = r.read_int64(bytes)?,
                Ok(82) => msg.nonce = r.read_bytes(bytes).map(Cow::Borrowed)?,
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
        } + if self.seed == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.seed).len())
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
        } + if self.timestamp == 0i64 {
            0
        } else {
            1 + sizeof_varint(*(&self.timestamp) as u64)
        } + if self.nonce == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.nonce).len())
        }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.account != Cow::Borrowed(b"") {
            w.write_with_tag(10, |w| w.write_bytes(&**&self.account))?;
        }
        if self.seed != Cow::Borrowed(b"") {
            w.write_with_tag(18, |w| w.write_bytes(&**&self.seed))?;
        }
        if self.encrypt_code != Cow::Borrowed(b"") {
            w.write_with_tag(26, |w| w.write_bytes(&**&self.encrypt_code))?;
        }
        if self.public_encrypt_type != "" {
            w.write_with_tag(34, |w| w.write_string(&**&self.public_encrypt_type))?;
        }
        if self.secret_encrypt_type != "" {
            w.write_with_tag(42, |w| w.write_string(&**&self.secret_encrypt_type))?;
        }
        if self.public_key != Cow::Borrowed(b"") {
            w.write_with_tag(50, |w| w.write_bytes(&**&self.public_key))?;
        }
        if self.secret_key != Cow::Borrowed(b"") {
            w.write_with_tag(58, |w| w.write_bytes(&**&self.secret_key))?;
        }
        if self.cert != Cow::Borrowed(b"") {
            w.write_with_tag(66, |w| w.write_bytes(&**&self.cert))?;
        }
        if self.timestamp != 0i64 {
            w.write_with_tag(72, |w| w.write_int64(*&self.timestamp))?;
        }
        if self.nonce != Cow::Borrowed(b"") {
            w.write_with_tag(82, |w| w.write_bytes(&**&self.nonce))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AccountMsg<'a> {
    pub account: Cow<'a, [u8]>,
    pub encrypt_code: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for AccountMsg<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.account = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.encrypt_code = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AccountMsg<'a> {
    fn get_size(&self) -> usize {
        0 + if self.account == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.account).len())
        } + if self.encrypt_code == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.encrypt_code).len())
        }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.account != Cow::Borrowed(b"") {
            w.write_with_tag(10, |w| w.write_bytes(&**&self.account))?;
        }
        if self.encrypt_code != Cow::Borrowed(b"") {
            w.write_with_tag(18, |w| w.write_bytes(&**&self.encrypt_code))?;
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
pub struct SignMsg<'a> {
    pub account_msg: Option<AccountMsg<'a>>,
    pub message: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for SignMsg<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.account_msg = Some(r.read_message::<AccountMsg>(bytes)?),
                Ok(18) => msg.message = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SignMsg<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .account_msg
            .as_ref()
            .map_or(0, |m| 1 + sizeof_len((m).get_size()))
            + if self.message == Cow::Borrowed(b"") {
                0
            } else {
                1 + sizeof_len((&self.message).len())
            }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.account_msg {
            w.write_with_tag(10, |w| w.write_message(s))?;
        }
        if self.message != Cow::Borrowed(b"") {
            w.write_with_tag(18, |w| w.write_bytes(&**&self.message))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PubVerifySign<'a> {
    pub public_key: Cow<'a, [u8]>,
    pub sign: Cow<'a, [u8]>,
    pub message: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for PubVerifySign<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.public_key = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.sign = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.message = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PubVerifySign<'a> {
    fn get_size(&self) -> usize {
        0 + if self.public_key == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.public_key).len())
        } + if self.sign == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.sign).len())
        } + if self.message == Cow::Borrowed(b"") {
            0
        } else {
            1 + sizeof_len((&self.message).len())
        }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.public_key != Cow::Borrowed(b"") {
            w.write_with_tag(10, |w| w.write_bytes(&**&self.public_key))?;
        }
        if self.sign != Cow::Borrowed(b"") {
            w.write_with_tag(18, |w| w.write_bytes(&**&self.sign))?;
        }
        if self.message != Cow::Borrowed(b"") {
            w.write_with_tag(26, |w| w.write_bytes(&**&self.message))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AccountVerifySign<'a> {
    pub sign_msg: Option<SignMsg<'a>>,
    pub sign: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for AccountVerifySign<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.sign_msg = Some(r.read_message::<SignMsg>(bytes)?),
                Ok(18) => msg.sign = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AccountVerifySign<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .sign_msg
            .as_ref()
            .map_or(0, |m| 1 + sizeof_len((m).get_size()))
            + if self.sign == Cow::Borrowed(b"") {
                0
            } else {
                1 + sizeof_len((&self.sign).len())
            }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.sign_msg {
            w.write_with_tag(10, |w| w.write_message(s))?;
        }
        if self.sign != Cow::Borrowed(b"") {
            w.write_with_tag(18, |w| w.write_bytes(&**&self.sign))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct VerifySign<'a> {
    pub VerfySign: mod_VerifySign::OneOfVerfySign<'a>,
}

impl<'a> MessageRead<'a> for VerifySign<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => {
                    msg.VerfySign = mod_VerifySign::OneOfVerfySign::AccountVerifySign(
                        r.read_message::<AccountVerifySign>(bytes)?,
                    )
                }
                Ok(18) => {
                    msg.VerfySign = mod_VerifySign::OneOfVerfySign::PubVerifySign(
                        r.read_message::<PubVerifySign>(bytes)?,
                    )
                }
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for VerifySign<'a> {
    fn get_size(&self) -> usize {
        0 + match self.VerfySign {
            mod_VerifySign::OneOfVerfySign::AccountVerifySign(ref m) => {
                1 + sizeof_len((m).get_size())
            }
            mod_VerifySign::OneOfVerfySign::PubVerifySign(ref m) => 1 + sizeof_len((m).get_size()),
            mod_VerifySign::OneOfVerfySign::None => 0,
        }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.VerfySign {
            mod_VerifySign::OneOfVerfySign::AccountVerifySign(ref m) => {
                w.write_with_tag(10, |w| w.write_message(m))?
            }
            mod_VerifySign::OneOfVerfySign::PubVerifySign(ref m) => {
                w.write_with_tag(18, |w| w.write_message(m))?
            }
            mod_VerifySign::OneOfVerfySign::None => {}
        }
        Ok(())
    }
}

pub mod mod_VerifySign {

    use super::*;
    use alloc::vec::Vec;

    #[derive(Debug, PartialEq, Clone)]
    pub enum OneOfVerfySign<'a> {
        AccountVerifySign(AccountVerifySign<'a>),
        PubVerifySign(PubVerifySign<'a>),
        None,
    }

    impl<'a> Default for OneOfVerfySign<'a> {
        fn default() -> Self {
            OneOfVerfySign::None
        }
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
