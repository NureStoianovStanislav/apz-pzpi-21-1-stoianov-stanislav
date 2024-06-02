create table users(
    id bigserial primary key,
    name varchar(50) not null,
    email varchar(50) not null unique,
    password_hash text not null,
    refresh_secret uuid not null unique
);
