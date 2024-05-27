use core::fmt;

use secrecy::{zeroize::DefaultIsZeroes, CloneableSecret, ExposeSecret, Secret, Zeroize};
use sqlx::{
    database::{HasArguments, HasValueRef},
    encode::IsNull,
    error::BoxDynError,
    prelude::Type,
    Decode, Encode, Postgres,
};

pub struct DbSecret<T>(Secret<Zeroizable<T>>)
where
    Zeroizable<T>: Zeroize;

#[derive(Clone, Copy, Default)]
pub struct Zeroizable<T>(T);

impl DefaultIsZeroes for Zeroizable<uuid::Uuid> {}
impl<T> CloneableSecret for Zeroizable<T>
where
    Self: Zeroize,
    T: Clone,
{
}

impl Zeroize for Zeroizable<String> {
    fn zeroize(&mut self) {
        self.0.zeroize()
    }
}

impl<T> DbSecret<T>
where
    Zeroizable<T>: Zeroize,
{
    pub fn new(value: T) -> Self {
        Self(Secret::new(Zeroizable(value)))
    }
}

impl<T> Clone for DbSecret<T>
where
    Zeroizable<T>: CloneableSecret,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> fmt::Debug for DbSecret<T>
where
    Zeroizable<T>: Zeroize,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Secret<{}>", std::any::type_name::<T>())
    }
}

impl<T> ExposeSecret<T> for DbSecret<T>
where
    Zeroizable<T>: Zeroize,
{
    fn expose_secret(&self) -> &T {
        &self.0.expose_secret().0
    }
}

impl<T> Type<Postgres> for DbSecret<T>
where
    Zeroizable<T>: Zeroize,
    T: Type<Postgres>,
{
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        <T>::type_info()
    }
}

impl<'q, T> Encode<'q, Postgres> for DbSecret<T>
where
    Zeroizable<T>: Zeroize,
    T: Encode<'q, Postgres>,
{
    fn encode_by_ref(&self, buf: &mut <Postgres as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
        self.expose_secret().encode_by_ref(buf)
    }
}

impl<'r, T> Decode<'r, Postgres> for DbSecret<T>
where
    Zeroizable<T>: Zeroize,
    T: Decode<'r, Postgres>,
{
    fn decode(value: <Postgres as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        <T>::decode(value).map(Self::new)
    }
}
