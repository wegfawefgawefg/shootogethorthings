use chrono::Utc;

pub fn get_utc_now() -> i64 {
    Utc::now().timestamp_millis()
}
