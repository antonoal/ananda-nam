create extension if not exists btree_gist;

create table streams (
    id serial not null,
    name text not null,
    valid_from timestamp not null,
    valid_to timestamp not null default 'infinity',
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),
    primary key (id, valid_from),
    exclude using gist (
        name with =,
        tsrange(valid_from, valid_to) with &&
    ),
    exclude using gist (id with =, tsrange(valid_from, valid_to) with &&)
);

create table privileges (
    id serial primary key,
    name text not null,
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now()
);

create table roles (
    id serial primary key,
    name text not null,
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now()
);

create table role_privileges (
    role_id int not null references roles (id),
    privilege_id int not null references privileges (id),
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),
    primary key (role_id, privilege_id)
);

create table persons (
    id serial not null,
    email text not null,
    password_hash text,
    name text not null,
    locked boolean not null,
    valid_from timestamp not null,
    valid_to timestamp not null default 'infinity',
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),
    primary key (id, valid_from),
    exclude using gist (id with =, tsrange(valid_from, valid_to) with &&),
    exclude using gist (
        email with =,
        tsrange(valid_from, valid_to) with &&
    )
);

create table person_roles (
    person_id int not null,
    person_valid_from timestamp not null,
    role_id int not null references roles (id),
    valid_from timestamp not null,
    valid_to timestamp not null default 'infinity',
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),
    primary key (
        person_id,
        person_valid_from,
        role_id,
        valid_from
    ),
    foreign key (person_id, person_valid_from) references persons (id, valid_from),
    exclude using gist (
        person_id with =,
        person_valid_from with =,
        role_id with =,
        tsrange(valid_from, valid_to) with &&
    )
);

select diesel_manage_updated_at('streams');

select diesel_manage_updated_at('privileges');

select diesel_manage_updated_at('roles');

select diesel_manage_updated_at('role_privileges');

select diesel_manage_updated_at('persons');

select diesel_manage_updated_at('person_roles');

insert into roles (name)
values ('ADMIN');

insert into persons (name, email, locked, valid_from)
values ('Superuser', 'a@a.a', false, now());

insert into person_roles (
        person_id,
        person_valid_from,
        role_id,
        valid_from
    )
select p.id,
    p.valid_from,
    r.id,
    now()
from persons p
    cross join roles r
where p.email = 'a@a.a'
    and r.name = 'ADMIN';

insert into "privileges" (name)
values ('EDIT_STRUCTURE'),
    ('ASSIGN_STUDENTS'),
    ('LOG_ATTENDANCE'),
    ('GENERATE_REPORTS'),
    ('VIEW_STUDENT');

insert into role_privileges (role_id, privilege_id)
select r.id,
    p.id
from roles r
    cross join "privileges" p
where r.name = 'ADMIN';