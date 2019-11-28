use chrono::prelude::*;

pub fn rfc3339_utc(d: &DateTime<Utc>) -> askama::Result<String> {
    Ok(d.to_rfc3339())
}

pub fn human_date(d: &DateTime<Utc>) -> askama::Result<String> {
    Ok(d.format("%b %e, %Y").to_string())
}

pub fn year(d: &DateTime<Utc>) -> askama::Result<String> {
    Ok(d.format("%Y").to_string())
}
