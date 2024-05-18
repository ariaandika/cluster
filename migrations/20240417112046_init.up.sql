-- Add up migration script here
create table users (
    user_id         int generated always as identity primary key,
    name            text not null unique check(length(name) between 3 and 50),
    phone           text not null unique check(length(phone) between 10 and 13),
    password        text not null,
    role            text not null,
    metadata        json not null default '{}'::json,
    created_at      timestamptz not null default now(),
    updated_at      timestamptz not null default now(),
    verified_at     timestamptz
);

create table users_snapshot (
    snapshot_id     int generated always as identity primary key,
    user_id         int, -- for anon user
    name            text not null,
    phone           text not null,
    role            text not null,
    metadata        json not null,
    snapshoted_at   timestamptz not null default now()
);

create table warehouses (
    wh_id           int generated always as identity primary key,
    name            text not null,
    wh_type         text not null,
    created_at      timestamptz not null default now(),
    updated_at      timestamptz not null default now()
);

create table warehouses_snapshot (
    snapshot_id     int generated always as identity primary key,
    wh_id           int, -- for anon / vendor warehouse ?
    name            text not null,
    type            text not null,
    snapshoted_at   timestamptz not null default now()
);

create table employees (
    user_id         int not null references users(user_id) primary key,
    wh_id           int not null references warehouses(wh_id)
);

create table orders (
    order_id        int generated always as identity primary key,
    sender_id       int not null,
    receiver_id     int not null,
    destination     json not null
);

create table tracings (
    tracing_id      int generated always as identity primary key,
    order_id        int not null unique, -- only one active order at a time
    status          text not null,
    subject_id      int not null,
    subject_name    text not null,
    created_at      timestamptz not null default now()
);

create table tracings_archive (
    tracing_id      int not null,
    order_id        int not null,
    status          text not null,
    subject_id      int not null,
    subject_name    text not null,
    created_at      timestamptz not null,
    archived_at     timestamptz not null default now()
);

create table packages (
    package_id      int generated always as identity primary key,
    order_id        int not null,
    name            text not null,
    weight          real not null,
    length          real not null,
    width           real not null,
    height          real not null
);

create table manifests (
    manifest_id     int generated always as identity primary key,
    sales_id        int not null,
    driver_id       int not null,
    wh_from_id      int not null,
    wh_to_id        int not null,
    created_at      timestamptz not null default now(),
    completed_at    timestamptz
);

create table manifest_orders (
    manifest_id     int not null,
    order_id        int not null
);

