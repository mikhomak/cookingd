CREATE TABLE "siteconfigurations"
(
    id                  SERIAL PRIMARY KEY,
    allow_site_comments BOOL NOT NULL default true,
    allow_posting       BOOL not null default true,
    allow_registration  BOOL not null default true,
    allow_login         BOOL not null default true
);

CREATE TABLE "c_user"
(
    id            UUID         NOT NULL DEFAULT gen_random_uuid(),
    name          VARCHAR(50)  NOT NULL,
    email         VARCHAR(254) NOT NULL,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT 'NOW'::timestamptz,
    password      VARCHAR(50)  NOT NULL,
    login_enabled bool         NOT NULL DEFAULT TRUE,
    consent       bool         NOT NULL DEFAULT FALSE,
    PRIMARY KEY (id)
);


CREATE TABLE "post"
(
    id             UUID        NOT NULL DEFAULT gen_random_uuid(),
    title          VARCHAR(50) NOT NULL,
    text           text,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT 'NOW'::timestamptz,
    likes          bigint      NOT NULL DEFAULT 0,
    rating         float8      NOT NULL DEFAULT 0,
    user_id        UUID        NOT NULL REFERENCES "c_user" (id),
    allow_comments BOOL        NOT NULL default true,
    allow_likes    BOOL        NOT NULL default true,
    PRIMARY KEY (id)
);

CREATE TABLE "comment"
(
    id         UUID        NOT NULL DEFAULT gen_random_uuid(),
    user_id    UUID        NOT NULL REFERENCES "c_user" (id),
    post_id    UUID        NOT NULL REFERENCES "post" (id),
    text       text        not null,
    created_at TIMESTAMPTZ NOT NULL DEFAULT 'NOW'::timestamptz,
    likes      bigint      NOT NULL DEFAULT 0,
    PRIMARY KEY (id)
);
