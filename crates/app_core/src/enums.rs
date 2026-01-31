use serde;
use sqlx;
macro_rules! define_enum {
    (
        $name:ident { $($variant:ident = $value:expr),* $(,)? }
    ) => {
        #[repr(i16)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[derive(serde::Serialize, serde::Deserialize)]
        pub enum $name {
            $($variant = $value),*
        }

        impl sqlx::Type<sqlx::Postgres> for $name {
            fn type_info() -> sqlx::postgres::PgTypeInfo {
                <i16 as sqlx::Type<sqlx::Postgres>>::type_info()
            }

            fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
                <i16 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
            }
        }

        impl<'r> sqlx::Decode<'r, sqlx::Postgres> for $name {
            fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
                let v = <i16 as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
                match v {
                    $($value => Ok(Self::$variant),)*
                    _ => Err(format!("invalid {} value: {}", stringify!($name), v).into()),
                }
            }
        }

        impl<'q> sqlx::Encode<'q, sqlx::Postgres> for $name {
            fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
                <i16 as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&(*self as i16), buf)
            }
        }

         impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}::{}", stringify!($name), match self {
                    $(Self::$variant => stringify!($variant),)*
                })
            }
        }

        impl TryFrom<i16> for $name {
            type Error = String;
            fn try_from(v: i16) -> Result<Self, Self::Error> {
                match v {
                    $($value => Ok(Self::$variant),)*
                    _ => Err(format!("invalid {} value: {}", stringify!($name), v).into()),
                }
            }
        }

    };
}

define_enum!(UserStatus {
    Active = 1,
    Suspended = 2,
    Deleted = 3,
});

define_enum!(OAuthProvider {
    Google = 1,
    Apple = 2,
    Telegram = 3,
    Github = 4,
    Twitter = 5,
    Yandex = 6,
    Reddit = 7,
});

define_enum!(VerificationType {
    EmailConfirm = 1,
    EmailLink = 2,
    PasswordReset = 3,
    PasswordSet = 4,
});

define_enum!(AuthEventType {
    Register = 1,
    Login = 2,
    Logout = 3,
    LogoutAll = 4,
    PasswordChange = 5,
    PasswordReset = 6,
    EmailChange = 7,
    EmailVerified = 8,
    OAuthLink = 9,
    OAuthUnlink = 10,
    AccountSuspended = 11,
    AccountDeleted = 12,
    AnonymousUpgrade = 13,
});
