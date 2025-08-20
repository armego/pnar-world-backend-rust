use crate::error::AppError;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

/// Utility for automatically tracking analytics events
pub struct AnalyticsTracker;

impl AnalyticsTracker {
    /// Track word usage analytics
    pub async fn track_word_usage(
        pool: &PgPool,
        word_id: Uuid,
        user_id: Option<Uuid>,
        usage_type: &str,
        session_id: Option<String>,
        ip_address: Option<String>,
        user_agent: Option<String>,
        context_data: Option<serde_json::Value>,
    ) -> Result<(), AppError> {
        let analytics_id = Uuid::new_v4();
        
        sqlx::query(
            r#"
            INSERT INTO word_usage_analytics (
                id, word_id, user_id, usage_type, context_data, 
                session_id, ip_address, user_agent, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())
            "#,
        )
        .bind(analytics_id)
        .bind(word_id)
        .bind(user_id)
        .bind(usage_type)
        .bind(context_data.unwrap_or_else(|| serde_json::json!({})))
        .bind(session_id)
        .bind(ip_address)
        .bind(user_agent)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e))?;

        Ok(())
    }

    /// Track user contributions
    pub async fn track_contribution(
        pool: &PgPool,
        user_id: Uuid,
        contribution_type: &str,
        entity_type: &str,
        entity_id: Uuid,
        action: &str,
        previous_value: Option<serde_json::Value>,
        new_value: Option<serde_json::Value>,
        points_awarded: i32,
    ) -> Result<(), AppError> {
        let contribution_id = Uuid::new_v4();
        
        sqlx::query(
            r#"
            INSERT INTO user_contributions (
                id, user_id, contribution_type, entity_type, entity_id,
                action, previous_value, new_value, points_awarded,
                status, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'approved', NOW())
            "#,
        )
        .bind(contribution_id)
        .bind(user_id)
        .bind(contribution_type)
        .bind(entity_type)
        .bind(entity_id)
        .bind(action)
        .bind(previous_value)
        .bind(new_value)
        .bind(points_awarded)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e))?;

        // Award points to user
        Self::award_points(pool, user_id, points_awarded).await?;

        Ok(())
    }

    /// Track dictionary search analytics
    pub async fn track_search(
        pool: &PgPool,
        query: &str,
        user_id: Option<Uuid>,
        session_id: Option<String>,
        results_count: usize,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<(), AppError> {
        // For searches, we track against the first result or create a generic search event
        let context_data = serde_json::json!({
            "search_query": query,
            "results_count": results_count,
            "search_timestamp": Utc::now()
        });

        // Create a search analytics record (without specific word_id for general searches)
        let analytics_id = Uuid::new_v4();
        
        sqlx::query(
            r#"
            INSERT INTO word_usage_analytics (
                id, word_id, user_id, usage_type, context_data,
                session_id, ip_address, user_agent, created_at
            )
            VALUES ($1, $2, $3, 'search', $4, $5, $6, $7, NOW())
            "#,
        )
        .bind(analytics_id)
        .bind(None::<Uuid>) // No specific word for general search
        .bind(user_id)
        .bind(context_data)
        .bind(session_id)
        .bind(ip_address)
        .bind(user_agent)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e))?;

        Ok(())
    }

    /// Award points to user and update their total
    async fn award_points(pool: &PgPool, user_id: Uuid, points: i32) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE users SET translation_points = translation_points + $1 WHERE id = $2"
        )
        .bind(points)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e))?;

        Ok(())
    }

    /// Calculate points for different contribution types
    pub fn calculate_contribution_points(contribution_type: &str, action: &str) -> i32 {
        match (contribution_type, action) {
            ("dictionary_entry", "create") => 10,
            ("dictionary_entry", "update") => 5,
            ("dictionary_entry", "verify") => 15,
            ("translation_request", "create") => 3,
            ("translation_request", "complete") => 8,
            ("dictionary_entry", "review") => 5,
            _ => 1, // Default points for any contribution
        }
    }
}
