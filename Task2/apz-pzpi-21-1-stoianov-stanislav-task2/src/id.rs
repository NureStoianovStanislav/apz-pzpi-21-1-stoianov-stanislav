use core::fmt;
use std::{str::FromStr, sync::OnceLock};

use aes::{
    cipher::{BlockDecrypt, BlockEncrypt},
    Aes128,
};
use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(Clone, Copy, SerializeDisplay, DeserializeFromStr)]
pub struct Id<const TAG: u64>(u128);

pub const fn tag(s: &str) -> u64 {
    let bytes = s.as_bytes();
    let mut result = 0u64;
    let mut i = 0;
    while i < bytes.len() {
        result |= (bytes[i] as u64) << (8 * i);
        i += 1;
    }
    result
}

impl<const TAG: u64> Id<TAG> {
    pub fn new(id: i64, cipher: &Aes128) -> Self {
        Self(encrypt(TAG, id, cipher))
    }

    pub fn sql_id(self, cipher: &Aes128) -> Result<i64, DecodeError> {
        decrypt(TAG, self.0, cipher)
    }
}

impl<const TAG: u64> fmt::Display for Id<TAG> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        encode(self.0, f)
    }
}

impl<const TAG: u64> fmt::Debug for Id<TAG> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Id").field(&format_args!("{self}")).finish()
    }
}

impl<const TAG: u64> FromStr for Id<TAG> {
    type Err = DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        decode(s).map(Self)
    }
}

fn encrypt(tag: u64, id: i64, cipher: &Aes128) -> u128 {
    let tagged = concat(tag, id);
    let mut bytes = tagged.into();
    cipher.encrypt_block(&mut bytes);
    u128::from_le_bytes(bytes.into())
}

fn decrypt(
    expected_tag: u64,
    id: u128,
    cipher: &Aes128,
) -> Result<i64, DecodeError> {
    let mut bytes = id.to_le_bytes().into();
    cipher.decrypt_block(&mut bytes);
    match bisect(bytes.into()) {
        (tag, id) if tag == expected_tag => Ok(id),
        _ => Err(DecodeError),
    }
}

fn alphabet() -> &'static str {
    static ALPHABET: OnceLock<String> = OnceLock::new();
    ALPHABET.get_or_init(|| ('a'..='z').chain('A'..='Z').collect())
}

fn encode(n: u128, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let base = alphabet().len() as u128;
    let bytes = alphabet().as_bytes();
    std::iter::successors(Some(n), |&n| match n / base {
        0 => None,
        d => Some(d),
    })
    .map(|i| bytes[(i % base) as usize] as char)
    .try_for_each(|c| write!(f, "{c}"))
}

#[derive(Debug, thiserror::Error)]
#[error("failed to parse id")]
pub struct DecodeError;

fn decode(s: &str) -> Result<u128, DecodeError> {
    let base = alphabet().len() as u128;
    s.chars()
        .map(|c| alphabet().chars().position(|a| c == a))
        .enumerate()
        .try_fold(0, |acc, (i, n)| match n {
            Some(n) => Ok(acc + n as u128 * base.pow(i as u32)),
            None => Err(DecodeError),
        })
}

fn concat(tag: u64, id: i64) -> [u8; 16] {
    let tag = (tag as u128).reverse_bits();
    let id = u64::from_le_bytes(id.to_le_bytes()) as u128;
    (tag | id).to_le_bytes()
}

fn bisect(bytes: [u8; 16]) -> (u64, i64) {
    const HIGH_BITS: u128 = !0 << (u128::BITS / 2);
    const LOW_BITS: u128 = !0 >> (u128::BITS / 2);
    let tagged = u128::from_le_bytes(bytes);
    let tag = (tagged & HIGH_BITS).reverse_bits() as u64;
    let id = (tagged & LOW_BITS) as u64;
    let id = i64::from_le_bytes(id.to_le_bytes());
    (tag, id)
}
