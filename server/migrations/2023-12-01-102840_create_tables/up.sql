create extension if not exists btree_gist;

/************** School structure ****************/
create table schools (
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

create table years (
    id serial not null,
    school_id int not null,
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

create table streams (
    id serial not null,
    school_id int not null,
    name text not null,
    start_year smallint not null CHECK (start_year > 0),
    -- end_year int not null CHECK (end_year > start_year),
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

/************** Group structure ****************/
create table groups (
    id serial not null,
    parent_id int,
    year_id int,
    stream_id int not null,
    name text not null,
    valid_from timestamp not null,
    valid_to timestamp not null default 'infinity',
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),
    primary key (id, valid_from),
    exclude using gist (id with =, tsrange(valid_from, valid_to) with &&)
);

create table group_roles (
    id serial primary key,
    name text not null unique,
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now()
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

create table group_persons (
    person_id int not null,
    group_id int not null,
    group_role_id int references group_roles (id),
    valid_from timestamp not null,
    valid_to timestamp,
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),
    primary key (person_id, group_id, valid_from),
    exclude using gist (
        person_id with =,
        group_id with =,
        tsrange(valid_from, valid_to) with &&
    )
);

/************** Permissions ****************/
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

create table person_roles (
    person_id int not null,
    role_id int not null references roles (id),
    school_id int,
    year_id int,
    stream_id int,
    group_id int,
    entity_id int generated always as (
        coalesce(school_id, year_id, stream_id, group_id)
    ) stored,
    entity_type text generated always as (
        case
            when school_id is not null then 'shool'
            when year_id is not null then 'year'
            when stream_id is not null then 'stream'
            when group_id is not null then 'group'
        end
    ) stored,
    valid_from timestamp not null,
    valid_to timestamp not null default 'infinity',
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),
    primary key (
        person_id,
        role_id,
        valid_from
    ),
    check (
        (
            case
                when school_id is null then 0
                else 1
            end + case
                when year_id is null then 0
                else 1
            end + case
                when stream_id is null then 0
                else 1
            end + case
                when group_id is null then 0
                else 1
            end + case
                when group_id is null then 0
                else 1
            end
        ) < 2
    ),
    exclude using gist (
        person_id with =,
        role_id with =,
        tsrange(valid_from, valid_to) with &&
    )
);

/************* Triggers *****************/
select diesel_manage_updated_at('schools');

select diesel_manage_updated_at('years');

select diesel_manage_updated_at('streams');

select diesel_manage_updated_at('groups');

select diesel_manage_updated_at('group_roles');

select diesel_manage_updated_at('persons');

select diesel_manage_updated_at('group_persons');

select diesel_manage_updated_at('privileges');

select diesel_manage_updated_at('roles');

select diesel_manage_updated_at('role_privileges');

select diesel_manage_updated_at('person_roles');

/************* Data *****************/
insert into roles (name)
values ('ADMIN');

insert into persons (name, email, password_hash, locked, valid_from)
values (
        'Superuser',
        'a@a.a',
        '$2b$12$Mocm80SGmT7j.wRX3tdD8el/wDYvt5SfoaxacblIeXvEGknMdiBbq',
        false,
        now()
    );

insert into person_roles (
        person_id,
        role_id,
        valid_from
    )
select p.id,
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

-- insert into group_roles(name)
-- values ('STUDENT');