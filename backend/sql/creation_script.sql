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


-- User lookup indices
CREATE INDEX idx_users_email ON chat.users(email);
CREATE INDEX idx_users_student_id ON chat.users(student_id);

-- Conversation lookup indices
CREATE INDEX idx_conversations_owner_id ON chat.conversations(owner_id);
CREATE INDEX idx_conversations_last_message_at ON chat.conversations(last_message_at);
CREATE INDEX idx_conversations_status ON chat.conversations(status);

-- Composite index for filtered conversation queries
CREATE INDEX idx_conversations_owner_status ON chat.conversations(owner_id, status);

-- Message lookup indices
CREATE INDEX idx_messages_conversation_id ON chat.messages(conversation_id);
CREATE INDEX idx_messages_role ON chat.messages(role);
CREATE INDEX idx_messages_created_at ON chat.messages(created_at);

-- Composite index for message retrieval in chronological order
CREATE INDEX idx_messages_conversation_created ON chat.messages(conversation_id, created_at);