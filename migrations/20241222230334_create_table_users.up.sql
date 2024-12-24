-- table groups
create table groups (
    id uuid primary key not null default gen_random_uuid(),
    name varchar(30) not null,
    description varchar(255),
    permissions jsonb not null default '[]',
    visible boolean not null default true,
    editable boolean not null default true,
    locked boolean not null default false,
    created_at bigint not null default extract(
        epoch
        from now()
    ),
    updated_at bigint not null default extract(
        epoch
        from now()
    ),
    deleted_at bigint,
    unique (name)
);
--
-- table users
--
create table users (
id uuid primary key not null default gen_random_uuid(),
name varchar(30) not null,
phone varchar(30),
role varchar(30),
email varchar(255),
username varchar(30) not null,
password_hash bytea not null,
visible boolean not null default true,
editable boolean not null default true,
locked boolean not null default false,
created_at bigint not null default extract(
    epoch
    from now()
),
updated_at bigint not null default extract(
    epoch
    from now()
),
deleted_at bigint,
unique (email),
unique (username)
);
--
-- pivot table users_groups
--
create table users_groups (
user_id uuid not null,
group_id uuid not null,
primary key (user_id, group_id),
foreign key (user_id) references users(id) on delete cascade,
foreign key (group_id) references groups(id) on delete cascade
);