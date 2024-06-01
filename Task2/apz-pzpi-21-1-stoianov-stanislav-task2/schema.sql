create table users(
    id bigserial primary key,
    email varchar(50) not null unique,
    password_hash text not null,
    refresh_token uuid not null unique
);
