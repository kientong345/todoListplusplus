use chrono::{DateTime, Duration, Utc};
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize, Serializer};
use sqlx::postgres::types::PgInterval;
use tokio::time::Instant;

/// Converts a vector of any type that implements ToString into a vector of strings
pub fn vec_stringify<T: ToString>(vec: Vec<T>) -> Vec<String> {
    let mut ret = Vec::new();
    for element in vec {
        ret.push(element.to_string());
    }
    ret
}

/// Validates if the provided email string matches a standard email format
pub fn validate_email_name(email: &str) -> Result<(), String> {
    let email_regex =
        regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if email_regex.is_match(email) {
        Ok(())
    } else {
        Err(String::from("Invalid email format"))
    }
}

/// Decodes a JWT token into the specified claims type using the provided secret
pub fn decode_jwt<C: Clone + DeserializeOwned>(
    jwt: &str,
    secret: &[u8],
) -> Result<C, jsonwebtoken::errors::Error> {
    Ok(jsonwebtoken::decode::<C>(
        jwt,
        &jsonwebtoken::DecodingKey::from_secret(&secret),
        &jsonwebtoken::Validation::default(),
    )?
    .claims)
}

/// Generates a JWT token from the specified claims type using the provided secret
pub fn generate_jwt<C: Serialize>(claims: &C, secret: &[u8]) -> String {
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(&secret),
    )
    .unwrap_or(String::from(""))
}

/// Hashes a given string using bcrypt and returns the hashed value
pub fn bcrypt_hash(value: &str) -> Result<String, bcrypt::BcryptError> {
    Ok(bcrypt::hash(value, bcrypt::DEFAULT_COST)?)
}

/// Verifies a given string against a bcrypt hashed value
#[allow(non_snake_case)]
pub fn serializeCamelCase<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
{
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Camel<'a, T: Serialize>(&'a T);

    Camel(value).serialize(serializer)
}

/// Deserializes a value from snake_case format
pub fn deserialize_snake_case<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(rename_all = "snake_case")]
    struct Snake<T>(T);

    Snake::deserialize(deserializer).map(|s| s.0)
}

/// Retrieves a query parameter from a map, with an optional default value
#[macro_export]
macro_rules! get_query_param {
    // With default value
    ($map:expr, $key:expr, $default:expr) => {
        $map.get($key)
            .and_then(|s| s.parse().ok())
            .unwrap_or($default)
    };

    // Without default value
    ($map:expr, $key:expr) => {
        $map.get($key).and_then(|s| s.parse().ok())
    };
}

pub fn get_runtime(
    thread_num: usize,
    thread_name: &str,
) -> Result<tokio::runtime::Runtime, std::io::Error> {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(thread_num)
        .thread_name(thread_name)
        .enable_all()
        .build()
}

pub fn datetime_to_string(datetime: DateTime<Utc>) -> String {
    datetime.to_rfc3339()
}

pub fn string_to_datetime(datetime: &str) -> DateTime<Utc> {
    println!("datetime: {}", datetime);

    // Try parsing as RFC3339 first
    if let Ok(dt) = DateTime::parse_from_rfc3339(datetime) {
        return dt.into();
    }

    // Try parsing as simple date YYYY-MM-DD by appending default time
    let datetime_with_time = format!("{}T00:00:00Z", datetime);
    if let Ok(dt) = DateTime::parse_from_rfc3339(&datetime_with_time) {
        return dt.into();
    }

    // If all else fails, panic (or you could return a Result if signature changed)
    panic!("Failed to parse datetime: {}", datetime);
}

pub fn pg_interval_to_string(interval: PgInterval) -> String {
    format!(
        "{} {} {}",
        interval.months, interval.days, interval.microseconds
    )
}

pub fn string_to_pg_interval(interval: &str) -> PgInterval {
    println!("interval: {}", interval);
    let parts: Vec<&str> = interval.split(' ').collect();
    let months = parts[0].parse::<i32>().unwrap();
    let days = parts[1].parse::<i32>().unwrap();
    let microseconds = parts[2].parse::<i64>().unwrap();
    PgInterval {
        months,
        days,
        microseconds,
    }
}

pub fn pg_interval_to_time(interval: PgInterval) -> Duration {
    let seconds_from_days = (interval.months * 30 + interval.days) * 24 * 3600;
    Duration::seconds(seconds_from_days as i64) + Duration::microseconds(interval.microseconds)
}

pub async fn sleep_until_dt(datetime: DateTime<Utc>) {
    tokio::time::sleep_until(Instant::now() + (datetime - Utc::now()).to_std().unwrap()).await
}
