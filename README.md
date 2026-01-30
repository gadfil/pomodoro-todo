#  Pomodoro Todo App

Workspace + Domain-based arhitecture 

```
pomodoro-todo/
├── Cargo.toml                    # workspace definition
├── .env.example
├── docker-compose.yml
├── migrations/                   # sqlx migrations
│
├── crates/
│   ├── api/                      # HTTP API (Axum)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── lib.rs
│   │       ├── router.rs         # route composition
│   │       ├── middleware/
│   │       │   ├── mod.rs
│   │       │   ├── auth.rs       # JWT/session extraction
│   │       │   └── rate_limit.rs
│   │       ├── handlers/
│   │       │   ├── mod.rs
│   │       │   ├── auth.rs       # login, oauth callbacks
│   │       │   ├── tasks.rs
│   │       │   ├── pomodoro.rs
│   │       │   └── telegram.rs   # link/unlink, webhook
│   │       ├── extractors/       # custom axum extractors
│   │       └── response.rs       # unified API response
│   │
│   ├── bot/                      # Telegram Bot
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── lib.rs
│   │       ├── handlers/
│   │       │   ├── mod.rs
│   │       │   ├── commands.rs   # /start, /tasks, /pomo
│   │       │   ├── callbacks.rs  # inline buttons
│   │       │   └── notifications.rs
│   │       ├── keyboard.rs       # inline keyboards builder
│   │       └── state.rs          # bot state, FSM
│   │
│   ├── domain/                   # Core business logic (shared)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── users/
│   │       │   ├── mod.rs
│   │       │   ├── models.rs     # User, UserProfile
│   │       │   ├── service.rs    # create, update, link_telegram
│   │       │   └── repository.rs # trait + queries
│   │       ├── auth/
│   │       │   ├── mod.rs
│   │       │   ├── models.rs     # Session, OAuthProvider, Token
│   │       │   ├── service.rs    # login, oauth flow, refresh
│   │       │   ├── oauth/
│   │       │   │   ├── mod.rs
│   │       │   │   ├── google.rs
│   │       │   │   ├── github.rs
│   │       │   │   └── traits.rs # OAuthProvider trait
│   │       │   └── jwt.rs
│   │       ├── tasks/
│   │       │   ├── mod.rs
│   │       │   ├── models.rs     # Task, TaskStatus, Priority
│   │       │   ├── service.rs
│   │       │   └── repository.rs
│   │       ├── pomodoro/
│   │       │   ├── mod.rs
│   │       │   ├── models.rs     # PomodoroSession, Settings
│   │       │   ├── service.rs    # start, pause, complete
│   │       │   ├── repository.rs
│   │       │   └── timer.rs      # timer logic, state machine
│   │       └── notifications/
│   │           ├── mod.rs
│   │           ├── models.rs     # Notification, Channel
│   │           └── service.rs    # send via telegram/push
│   │
│   ├── infrastructure/           # External integrations
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── db/
│   │       │   ├── mod.rs
│   │       │   ├── postgres.rs   # pool setup, health check
│   │       │   └── redis.rs      # sessions, rate limiting
│   │       ├── telegram/
│   │       │   ├── mod.rs
│   │       │   └── client.rs     # send messages API
│   │       └── email/
│   │           ├── mod.rs
│   │           └── sender.rs     # verification emails
│   │
│   └── shared/                   # Common utilities
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── config.rs         # typed config from env
│           ├── errors.rs         # AppError, error types
│           ├── dto/              # shared request/response types
│           │   ├── mod.rs
│           │   ├── auth.rs
│           │   ├── tasks.rs
│           │   └── pomodoro.rs
│           └── utils/
│               ├── mod.rs
│               ├── time.rs
│               └── validation.rs
│
└── tests/
    ├── api/                      # integration tests for API
    └── bot/                      # integration tests for bot

    ```