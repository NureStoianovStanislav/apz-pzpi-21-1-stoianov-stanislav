#[derive(Clone, Copy, Debug, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
pub enum Role {
    Administrator,
    Client,
}
