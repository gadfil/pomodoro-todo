#  Pomodoro Todo App

Workspace with 3 crates: `app_core`, `api`, `bot`.

```
pomodoro-todo/
├── Cargo.toml                    # workspace definition
├── .env.example
├── docker-compose.yml
├── migrations/                   # sqlx migrations
│
├── crates/
│   ├── app_core/                 # Config, models, DB, state, errors
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── config.rs         # typed config from env
│   │       ├── enums.rs          # UserStatus, OAuthProvider, etc.
│   │       ├── errors.rs         # StateError
│   │       ├── state.rs          # AppState (PgPool + Config)
│   │       ├── dto/
│   │       │   ├── mod.rs
│   │       │   └── auth.rs       # RegisterRequest/Response
│   │       └── db/
│   │           ├── mod.rs
│   │           └── postgres.rs   # pool setup
│   │
│   ├── api/                      # HTTP API (Axum)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── routers/
│   │       │   ├── mod.rs
│   │       │   └── auth.rs
│   │       └── handlers/
│   │           ├── mod.rs
│   │           └── auth.rs
│   │
│   └── bot/                      # Telegram Bot
│       ├── Cargo.toml
│       └── src/
│
└── tests/
    ├── api/                      # integration tests for API
    └── bot/                      # integration tests for bot
```
