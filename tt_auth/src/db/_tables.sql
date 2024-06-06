create table permission_menu (
    `code` varchar(255) primary key,
    `name` varchar(99) not null,
    `is_reserved` bool not null,
    `update_time` timestamp not null default current_timestamp,
    `create_time` timestamp not null default current_timestamp
)

create table permission (
    `code` varchar(255) primary key,
    `menu_code` varchar(255) not null,
    `name` varchar(99) not null,
    `is_reserved` bool not null,
    `update_time` timestamp not null default current_timestamp,
    `create_time` timestamp not null default current_timestamp,
    index index_menu_code (`menu_code`)
)

create table user_role (
    `id` bigint unsigned auto_increment primary key,
    `name` varchar(99) not null,
    `desc` varchar(99),
    `permission_codes_formatted` text,
    `is_reserved` bool not null,
    `status` tinyint unsigned not null default 1,
    `update_time` timestamp not null default current_timestamp,
    `create_time` timestamp not null default current_timestamp
)

create table user_role_group (
    `id` bigint unsigned auto_increment primary key,
    `name` varchar(99) not null,
    `desc` varchar(99),
    `role_ids_formatted` text,

    `is_reserved` bool not null,
    `status` tinyint unsigned not null default 1,
    `update_time` timestamp not null default current_timestamp,
    `create_time` timestamp not null default current_timestamp
)

create table user (
    `id` bigint unsigned auto_increment primary key,
    `role_group_ids_formatted` text,

    `account` varchar(99) unique null,
    `phone` varchar(99) unique null,
    `email` varchar(199) unique null,
    `wx_open_id` varchar(99) unique null,

    `nickname` varchar(99),
    `gender` tinyint unsigned,
    `avator` varchar(255),

    `password` varchar(99),

    `is_reserved` bool not null,
    `status` tinyint unsigned not null default 1,
    `update_time` timestamp not null default current_timestamp,
    `create_time` timestamp not null default current_timestamp

)



