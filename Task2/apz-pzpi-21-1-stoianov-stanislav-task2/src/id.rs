use core::fmt;
use std::{str::FromStr, sync::OnceLock};

use aes::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};
use secrecy::{ExposeSecret, Secret};
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
    pub fn new(id: u64, key: &Secret<[u8; 16]>) -> Self {
        Self(encrypt(TAG, id, key))
    }

    pub fn to_u64(self, key: &Secret<[u8; 16]>) -> u64 {
        decrypt(self.0, key)
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

fn encrypt(tag: u64, id: u64, key: &Secret<[u8; 16]>) -> u128 {
    let cipher = aes::Aes128::new(key.expose_secret().into());
    let tagged = (tag as u128) << 64 | id as u128;
    let mut bytes = tagged.to_le_bytes().into();
    cipher.encrypt_block(&mut bytes);
    u128::from_le_bytes(bytes.into())
}

fn decrypt(id: u128, key: &Secret<[u8; 16]>) -> u64 {
    let cipher = aes::Aes128::new(key.expose_secret().into());
    let mut bytes = id.to_le_bytes().into();
    cipher.decrypt_block(&mut bytes);
    u128::from_le_bytes(bytes.into()) as u64
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
