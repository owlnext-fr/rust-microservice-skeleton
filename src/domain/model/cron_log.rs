use crate::domain::schema::*;
use chrono::{DateTime, Utc};
use diesel::{Identifiable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Identifiable, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = cron_logs)]
pub struct CronLog {
    pub id: i32,
    pub command: String,
    pub command_args: String,
    pub exit_status: Option<i32>,
    pub exit_message: Option<String>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}
