
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "citext";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ============================================================================
-- Lookup Tables
-- ============================================================================

CREATE TABLE ref_oauth_providers (
                                     id   SMALLINT PRIMARY KEY,
                                     name VARCHAR(32) NOT NULL UNIQUE
);

INSERT INTO ref_oauth_providers (id, name) VALUES
                                               (1, 'google'),
                                               (2, 'apple'),
                                               (3, 'telegram'),
                                               (4, 'github'),
                                               (5, 'twitter'),
                                               (6, 'yandex'),
                                               (7, 'reddit');

CREATE TABLE ref_user_statuses (
                                   id   SMALLINT PRIMARY KEY,
                                   name VARCHAR(32) NOT NULL UNIQUE
);

INSERT INTO ref_user_statuses (id, name) VALUES
                                             (1, 'active'),
                                             (2, 'suspended'),
                                             (3, 'deleted');

CREATE TABLE ref_verification_types (
                                        id   SMALLINT PRIMARY KEY,
                                        name VARCHAR(32) NOT NULL UNIQUE
);

INSERT INTO ref_verification_types (id, name) VALUES
                                                  (1, 'email_confirm'),
                                                  (2, 'email_link'),
                                                  (3, 'password_reset'),
                                                  (4, 'password_set');

CREATE TABLE ref_auth_event_types (
                                      id   SMALLINT PRIMARY KEY,
                                      name VARCHAR(32) NOT NULL UNIQUE
);

INSERT INTO ref_auth_event_types (id, name) VALUES
                                                (1,  'register'),
                                                (2,  'login'),
                                                (3,  'logout'),
                                                (4,  'logout_all'),
                                                (5,  'password_change'),
                                                (6,  'password_reset'),
                                                (7,  'email_change'),
                                                (8,  'email_verified'),
                                                (9,  'oauth_link'),
                                                (10, 'oauth_unlink'),
                                                (11, 'account_suspended'),
                                                (12, 'account_deleted'),
                                                (13, 'anonymous_upgrade');

CREATE TABLE users (
                       id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

                       email          CITEXT,
                       email_verified BOOLEAN      NOT NULL DEFAULT FALSE,
                       password_hash  VARCHAR(255),

                       username       CITEXT,
                       display_name   VARCHAR(255) NOT NULL,
                       avatar_url     TEXT,

                       status         SMALLINT     NOT NULL DEFAULT 1 REFERENCES ref_user_statuses(id),
                       is_anonymous   BOOLEAN      NOT NULL DEFAULT FALSE,

                       created_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
                       updated_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE TABLE oauth_accounts (
                                id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                                user_id             UUID     NOT NULL REFERENCES users(id) ON DELETE CASCADE,

                                provider            SMALLINT NOT NULL REFERENCES ref_oauth_providers(id),
                                provider_account_id VARCHAR(255) NOT NULL,

                                access_token_enc    BYTEA,
                                refresh_token_enc   BYTEA,
                                token_expires_at    TIMESTAMPTZ,

                                provider_data       JSONB    NOT NULL DEFAULT '{}',

                                created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                                updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),

                                UNIQUE (provider, provider_account_id)
);

CREATE TABLE sessions (
                          id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                          user_id       UUID         NOT NULL REFERENCES users(id) ON DELETE CASCADE,

                          token_hash    VARCHAR(64)  NOT NULL UNIQUE,

                          device_name   VARCHAR(255),
                          user_agent    TEXT,
                          ip_address    INET,

                          created_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
                          expires_at    TIMESTAMPTZ  NOT NULL,
                          last_used_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

                          revoked       BOOLEAN      NOT NULL DEFAULT FALSE,
                          revoked_at    TIMESTAMPTZ
);

CREATE TABLE verifications (
                               id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                               user_id      UUID     NOT NULL REFERENCES users(id) ON DELETE CASCADE,

                               type         SMALLINT NOT NULL REFERENCES ref_verification_types(id),

                               code         VARCHAR(6),
                               token        VARCHAR(64),
                               new_email    CITEXT,

                               attempts     INT      NOT NULL DEFAULT 0,
                               max_attempts INT      NOT NULL DEFAULT 5,

                               expires_at   TIMESTAMPTZ NOT NULL,

                               used         BOOLEAN     NOT NULL DEFAULT FALSE,
                               used_at      TIMESTAMPTZ,

                               created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),

                               CONSTRAINT chk_code_or_token CHECK (code IS NOT NULL OR token IS NOT NULL),
                               CONSTRAINT chk_reasonable_expiry CHECK (expires_at <= created_at + INTERVAL '24 hours')
);

CREATE TABLE telegram_link_codes (
                                     id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

                                     telegram_id         BIGINT       NOT NULL,
                                     telegram_username   VARCHAR(255),
                                     telegram_first_name VARCHAR(255),
                                     telegram_photo_url  TEXT,

                                     code                VARCHAR(6)   NOT NULL,

                                     attempts            INT          NOT NULL DEFAULT 0,
                                     max_attempts        INT          NOT NULL DEFAULT 5,

                                     expires_at          TIMESTAMPTZ  NOT NULL,

                                     used                BOOLEAN      NOT NULL DEFAULT FALSE,
                                     used_by_user_id     UUID         REFERENCES users(id) ON DELETE SET NULL,
                                     used_at             TIMESTAMPTZ,

                                     created_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

                                     CONSTRAINT chk_tg_expiry CHECK (expires_at <= created_at + INTERVAL '1 hour')
);

CREATE TABLE auth_events (
                             id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                             user_id    UUID REFERENCES users(id) ON DELETE SET NULL,

                             event_type SMALLINT NOT NULL REFERENCES ref_auth_event_types(id),
                             provider   SMALLINT REFERENCES ref_oauth_providers(id),

                             ip_address INET,
                             user_agent TEXT,
                             metadata   JSONB NOT NULL DEFAULT '{}',

                             created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- Indexes
-- ============================================================================

CREATE UNIQUE INDEX idx_users_email ON users (email) WHERE email IS NOT NULL;
CREATE UNIQUE INDEX idx_users_username ON users (username) WHERE username IS NOT NULL;
CREATE INDEX idx_users_status ON users (status) WHERE status != 1;
CREATE INDEX idx_users_anonymous ON users (created_at) WHERE is_anonymous;

CREATE INDEX idx_oauth_user_id ON oauth_accounts (user_id);

CREATE INDEX idx_sessions_user ON sessions (user_id, created_at DESC) WHERE NOT revoked;
CREATE INDEX idx_sessions_token ON sessions (token_hash) WHERE NOT revoked;
CREATE INDEX idx_sessions_cleanup ON sessions (expires_at) WHERE NOT revoked;

CREATE INDEX idx_verif_user ON verifications (user_id, type) WHERE NOT used;
CREATE INDEX idx_verif_code ON verifications (user_id, code) WHERE NOT used AND code IS NOT NULL;
CREATE INDEX idx_verif_token ON verifications (token) WHERE NOT used AND token IS NOT NULL;
CREATE INDEX idx_verif_cleanup ON verifications (expires_at) WHERE NOT used;

CREATE INDEX idx_tg_link_code ON telegram_link_codes (code) WHERE NOT used;
CREATE INDEX idx_tg_link_telegram_id ON telegram_link_codes (telegram_id, created_at DESC);
CREATE INDEX idx_tg_link_cleanup ON telegram_link_codes (expires_at) WHERE NOT used;

CREATE INDEX idx_auth_events_user ON auth_events (user_id, created_at DESC);
CREATE INDEX idx_auth_events_type ON auth_events (event_type, created_at DESC);

-- ============================================================================
-- Functions
-- ============================================================================

CREATE OR REPLACE FUNCTION update_updated_at()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION generate_code_6()
    RETURNS VARCHAR(6) AS $$
BEGIN
    RETURN LPAD(FLOOR(RANDOM() * 1000000)::TEXT, 6, '0');
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION generate_token_64()
    RETURNS VARCHAR(64) AS $$
BEGIN
    RETURN ENCODE(gen_random_bytes(32), 'hex');
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- Triggers
-- ============================================================================

CREATE TRIGGER trg_users_updated
    BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER trg_oauth_updated
    BEFORE UPDATE ON oauth_accounts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

-- ============================================================================
-- Views (для удобного просмотра с расшифровкой)
-- ============================================================================

CREATE VIEW v_users_readable AS
SELECT
    u.id,
    u.email,
    u.email_verified,
    u.username,
    u.display_name,
    u.avatar_url,
    s.name AS status,
    u.is_anonymous,
    u.created_at,
    u.updated_at
FROM users u
         JOIN ref_user_statuses s ON s.id = u.status;

CREATE VIEW v_users_full AS
SELECT
    u.*,
    s.name AS status_name,
    COALESCE(
            (SELECT jsonb_agg(jsonb_build_object(
                    'provider', p.name,
                    'provider_account_id', oa.provider_account_id,
                    'provider_data', oa.provider_data,
                    'created_at', oa.created_at
                              ))
             FROM oauth_accounts oa
                      JOIN ref_oauth_providers p ON p.id = oa.provider
             WHERE oa.user_id = u.id),
            '[]'::jsonb
    ) AS oauth_accounts,
    (SELECT COUNT(*) FROM oauth_accounts WHERE user_id = u.id) AS oauth_count,
    (u.password_hash IS NOT NULL) AS has_password,
    (SELECT COUNT(*) FROM sessions WHERE user_id = u.id AND NOT revoked AND expires_at > NOW()) AS active_sessions
FROM users u
         JOIN ref_user_statuses s ON s.id = u.status;

CREATE VIEW v_auth_events_readable AS
SELECT
    e.id,
    e.user_id,
    t.name AS event_type,
    p.name AS provider,
    e.ip_address,
    e.user_agent,
    e.metadata,
    e.created_at
FROM auth_events e
         JOIN ref_auth_event_types t ON t.id = e.event_type
         LEFT JOIN ref_oauth_providers p ON p.id = e.provider;