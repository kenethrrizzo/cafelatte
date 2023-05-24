CREATE DATABASE salvo_skeleton;

USE salvo_skeleton;

CREATE TABLE `user` (
    `id` int primary key auto_increment,
    `name` varchar(40) not null,
    `surname` varchar(40) not null
);