use crate::domain::user::UserResponse;
use crate::model::request::registration::RegistrationRequest;
use crate::model::response::ErrorCode;
use crate::model::response::ErrorCode::{INTERNAL001, REG002};
use argon2::password_hash::SaltString;
use argon2::{password_hash, Argon2, PasswordHash, PasswordHasher};
use rand_core::OsRng;
use sqlx::PgPool;

#[doc = r#"
Register a user to the application.

If the provided email exists, ErrorCode::REG002 is returned.
If any other error occurres, ErrorCode::REG003 is returned.

After the user is created, the details are returned (excluding the password).
"#]
pub async fn register_new_user(
    data: RegistrationRequest,
    db: &PgPool,
) -> Result<UserResponse, ErrorCode> {
    // Check if the provided email is in use
    if sqlx::query!(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM users u
            WHERE LOWER(u.email) = LOWER($1)
        )
        "#,
        data.email
    )
    .fetch_one(db)
    .await
    .unwrap()
    .exists
    .unwrap()
    {
        return Err(REG002);
    }

    // Hash the password
    let salt: SaltString = SaltString::generate(&mut OsRng);
    let hashed_password: password_hash::Result<PasswordHash> =
        Argon2::default().hash_password(data.password.as_bytes(), &salt);

    match hashed_password {
        Ok(password) => Ok(sqlx::query_as!(
            UserResponse,
            r"
            INSERT INTO users (id, email, password)
            VALUES (gen_random_uuid(), $1, $2)
            RETURNING id, email, role, created_at
            ",
            data.email,
            password.to_string(),
        )
        .fetch_one(db)
        .await
        .unwrap()),
        Err(_) => Err(INTERNAL001),
    }
}
