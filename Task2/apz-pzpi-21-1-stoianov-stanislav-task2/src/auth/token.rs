use uuid::Uuid;

use crate::database::DbSecret;

pub type RefreshToken = DbSecret<Uuid>;

impl RefreshToken {
    pub fn generate() -> Self {
        DbSecret::new(Uuid::new_v4())
    }
}
