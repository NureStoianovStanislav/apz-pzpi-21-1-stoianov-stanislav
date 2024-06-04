create table users(
    id bigserial primary key,
    name varchar(50) not null,
    email varchar(50) not null unique,
    password_hash text not null,
    refresh_secret uuid not null unique,
    role varchar(32) not null
     check(role in ('administrator', 'client'))
);

create table libraries(
    id bigserial primary key,
    name varchar(50) not null,
    address varchar(100) not null,
    owner_id bigint not null
      references users(id)
      on delete cascade
);

create table books(
    id bigserial primary key,
    year smallint not null,
    name varchar(50) not null,
    genre varchar(50) not null,
    author varchar(50) not null,
    library_id bigint not null
      references libraries(id)
      on delete cascade
);
