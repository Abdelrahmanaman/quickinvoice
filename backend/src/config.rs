use chrono::Duration;

// JWT Durations
pub const ACCESS_TOKEN_TIME: Duration = Duration::minutes(15);

pub const REFRESH_TOKEN_TIME: Duration = Duration::days(14);
