/**
 * Function used to get a value of the .env config file. If the value isn't found then it searches throught the environmental parameters and if after that it doesn't find it then returns a None.
 */
pub fn get_value_from_key(key : &str) -> Option<String> {
    match dotenv::var(key){
        Ok(value) => Some(value),
        Err(_) => match std::env::var(key) {
            Ok(env_value) => Some(env_value),
            Err(_) => None
        }
    }
}

/**
 * Link for the datetime converter: https://github.com/Keats/jsonwebtoken/blob/master/examples/custom_chrono.rs
 */
pub mod jwt_numeric_date {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(date.timestamp())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Utc.timestamp_opt(i64::deserialize(deserializer)?, 0)
            .single()
            .ok_or_else(|| serde::de::Error::custom("invalid Unix timestamp value"))
    }
}