-- Add down migration script here
drop table if exists users;
drop table if exists users_snapshot;
drop table if exists warehouses;
drop table if exists warehouses_snapshot;
drop table if exists employees;
drop table if exists orders;
drop table if exists tracings;
drop table if exists tracings_archive;
drop table if exists packages;
drop table if exists manifests;
drop table if exists manifest_orders;
