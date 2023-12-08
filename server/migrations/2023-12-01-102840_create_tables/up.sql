create extension if not exists btree_gist;

create table persons (
    id serial not null,
    email text not null,
    password_hash text not null,
    name text not null,
    valid_from timestamp not null,
    valid_to timestamp default 'infinity',
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),
    primary key (id, valid_from),
    exclude using gist (id with =, tsrange(valid_from, valid_to) with &&),
    exclude using gist (
        email with =,
        tsrange(valid_from, valid_to) with &&
    )
);