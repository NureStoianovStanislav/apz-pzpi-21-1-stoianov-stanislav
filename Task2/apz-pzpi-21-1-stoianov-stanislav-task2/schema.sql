create table users(
    id bigserial primary key,
    email varchar(50) not null unique,
    password_hash char(100) not null,
    refresh_token char(37) not null unique
);
