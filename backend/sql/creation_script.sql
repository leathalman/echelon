create schema if not exists chat;

create type chat.message_role as enum ('user', 'assistant');

create type chat.conversation_status as enum ('active', 'archived', 'starred', 'system');

create table chat.users
(
    id               serial
        primary key,
    student_id       varchar(50)
        unique,
    email            varchar(255)                                       not null
        unique,
    password_hash    varchar(255)                                       not null,
    created_at       timestamp with time zone default CURRENT_TIMESTAMP not null,
    last_login_at    timestamp with time zone default CURRENT_TIMESTAMP not null,
    first_name       varchar(100),
    last_name        varchar(100),
    university       varchar(255),
    academic_profile text
);

create table chat.conversations
(
    id              serial
        primary key,
    owner_id        integer                                                             not null
        references chat.users,
    title           varchar(255)                                                        not null,
    last_message_at timestamp with time zone default CURRENT_TIMESTAMP                  not null,
    status          chat.conversation_status default 'active'::chat.conversation_status not null
);

create table chat.messages
(
    id              serial
        primary key,
    conversation_id integer                                            not null
        references chat.conversations,
    content         text                                               not null,
    role            chat.message_role                                  not null,
    created_at      timestamp with time zone default CURRENT_TIMESTAMP not null
);
