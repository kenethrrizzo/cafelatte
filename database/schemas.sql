drop database if exists `cafelatte`;

create database `cafelatte`;

use `cafelatte`;

create table `user_role`
(
    `code`        char        not null,
    `description` varchar(20) not null
);

insert into `user_role`
values ('A', 'Administrador');
insert into `user_role`
values ('E', 'Empleado');
insert into `user_role`
values ('C', 'Cliente');

create table `user`
(
    `id`           int primary key auto_increment,
    `name`         varchar(50)  not null,
    `surname`      varchar(50)  not null,
    `phone_number` varchar(20),
    `email`        varchar(50)  not null,
    `password`     varchar(100) not null,
    `role`         char         not null,
    `status`       char default 'V'
);

create table `address`
(
    `id`       int auto_increment,
    `user`     int          not null,
    `province` varchar(30)  not null,
    `city`     varchar(30)  not null,
    `address`  varchar(100) not null,
    primary key (`id`, `user`)
);

create table `payment_method`
(
    `code`        varchar(5)  not null,
    `description` varchar(20) not null,
    `enabled`     bool default true
);