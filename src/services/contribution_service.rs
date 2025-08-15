use crate::{
    dto::{responses::ContributionResponse, CreateContributionRequest, UpdateContributionRequest},
    error::AppError,
};
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub async fn create_contribution(
    pool: &PgPool,
    user_id: Uuid,
    request: CreateContributionRequest,
) -> Result<ContributionResponse, AppError> {
    let contribution_id = Uuid::new_v4();

    let record = sqlx::query(
        r#"
        INSERT INTO user_contributions (
            id, user_id, contribution_type, entity_type, entity_id, action,
            previous_value, new_value, points_awarded, status, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW())
        RETURNING id, user_id, contribution_type, entity_type, entity_id, action,
                  previous_value, new_value, points_awarded, status, reviewed_by, reviewed_at,
                  created_at
        "#,
    )
    .bind(contribution_id)
    .bind(user_id)
    .bind(&request.contribution_type)
    .bind(&request.entity_type)
    .bind(&request.entity_id)
    .bind(&request.action)
    .bind(&request.previous_value)
    .bind(&request.new_value)
    .bind(request.points_awarded.unwrap_or(0))
    .bind("pending".to_string())
    .fetch_one(pool)
    .await?;

    Ok(ContributionResponse {
        id: record.get("id"),
        user_id: record.get("user_id"),
        contribution_type: record.get("contribution_type"),
        entity_type: record.get("entity_type"),
        entity_id: record.get("entity_id"),
        action: record.get("action"),
        previous_value: record.get("previous_value"),
        new_value: record.get("new_value"),
        points_awarded: record.get("points_awarded"),
        status: record.get("status"),
        reviewed_by: record.get("reviewed_by"),
        reviewed_at: record.get("reviewed_at"),
        created_at: record.get("created_at"),
    })
}

pub async fn get_contribution(
    pool: &PgPool,
    contribution_id: Uuid,
    user_id: Uuid,
) -> Result<ContributionResponse, AppError> {
    let record = sqlx::query(
        r#"
        SELECT id, user_id, contribution_type, entity_type, entity_id, action,
               previous_value, new_value, points_awarded, status, reviewed_by, reviewed_at,
               created_at
        FROM user_contributions 
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(contribution_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    let record = record.ok_or_else(|| AppError::NotFound("Contribution not found".to_string()))?;

    Ok(ContributionResponse {
        id: record.get("id"),
        user_id: record.get("user_id"),
        contribution_type: record.get("contribution_type"),
        entity_type: record.get("entity_type"),
        entity_id: record.get("entity_id"),
        action: record.get("action"),
        previous_value: record.get("previous_value"),
        new_value: record.get("new_value"),
        points_awarded: record.get("points_awarded"),
        status: record.get("status"),
        reviewed_by: record.get("reviewed_by"),
        reviewed_at: record.get("reviewed_at"),
        created_at: record.get("created_at"),
    })
}

pub async fn list_contributions(
    pool: &PgPool,
    user_id: Option<Uuid>,
    page: i64,
    per_page: i64,
) -> Result<Vec<ContributionResponse>, AppError> {
    let offset = (page - 1) * per_page;

    let records = if let Some(uid) = user_id {
        sqlx::query(
            r#"
            SELECT id, user_id, contribution_type, entity_type, entity_id, action,
                   previous_value, new_value, points_awarded, status, reviewed_by, reviewed_at,
                   created_at
            FROM user_contributions 
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(uid)
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query(
            r#"
            SELECT id, user_id, contribution_type, entity_type, entity_id, action,
                   previous_value, new_value, points_awarded, status, reviewed_by, reviewed_at,
                   created_at
            FROM user_contributions 
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool)
        .await?
    };

    Ok(records
        .into_iter()
        .map(|record| ContributionResponse {
            id: record.get("id"),
            user_id: record.get("user_id"),
            contribution_type: record.get("contribution_type"),
            entity_type: record.get("entity_type"),
            entity_id: record.get("entity_id"),
            action: record.get("action"),
            previous_value: record.get("previous_value"),
            new_value: record.get("new_value"),
            points_awarded: record.get("points_awarded"),
            status: record.get("status"),
            reviewed_by: record.get("reviewed_by"),
            reviewed_at: record.get("reviewed_at"),
            created_at: record.get("created_at"),
        })
        .collect())
}

pub async fn update_contribution(
    pool: &PgPool,
    contribution_id: Uuid,
    user_id: Uuid,
    request: UpdateContributionRequest,
) -> Result<ContributionResponse, AppError> {
    let record = sqlx::query(
        r#"
        UPDATE user_contributions 
        SET 
            status = COALESCE($3, status)
        WHERE id = $1 AND user_id = $2
        RETURNING id, user_id, contribution_type, entity_type, entity_id, action,
                  previous_value, new_value, points_awarded, status, reviewed_by, reviewed_at,
                  created_at
        "#,
    )
    .bind(contribution_id)
    .bind(user_id)
    .bind(&request.status)
    .fetch_optional(pool)
    .await?;

    let record = record.ok_or_else(|| AppError::NotFound("Contribution not found".to_string()))?;

    Ok(ContributionResponse {
        id: record.get("id"),
        user_id: record.get("user_id"),
        contribution_type: record.get("contribution_type"),
        entity_type: record.get("entity_type"),
        entity_id: record.get("entity_id"),
        action: record.get("action"),
        previous_value: record.get("previous_value"),
        new_value: record.get("new_value"),
        points_awarded: record.get("points_awarded"),
        status: record.get("status"),
        reviewed_by: record.get("reviewed_by"),
        reviewed_at: record.get("reviewed_at"),
        created_at: record.get("created_at"),
    })
}

pub async fn delete_contribution(
    pool: &PgPool,
    contribution_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM user_contributions WHERE id = $1 AND user_id = $2")
        .bind(contribution_id)
        .bind(user_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Contribution not found".to_string()));
    }

    Ok(())
}
