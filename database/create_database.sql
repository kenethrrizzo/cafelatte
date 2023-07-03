drop database if exists `salvo_skeleton`;

create database `salvo_skeleton`;

use `salvo_skeleton`;

create table `user` (
    `id` int primary key auto_increment,
    `name` varchar(40) not null,
    `surname` varchar(40) not null,
    `phone_number` varchar(40),
    `email` varchar(50) not null,
    `password` varchar(100) not null
);

insert into `user` 
    (`name`, `surname`, `phone_number`, `email`, `password`)
values
    ('Keneth', 'Riera', '0988059308', 'kenethriera@gmail.com', 'test');