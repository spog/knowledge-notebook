// src/utils.rs
use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};

const FORMAT: &str = "%Y-%m-%d %H:%M:%S %Z"; // e.g. 2025-09-29 14:11:43 UTC

pub mod serde_datetime {
    use super::*;

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.format(FORMAT).to_string();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_str(&s, FORMAT)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use serde_json;

    use chrono::DateTime;

    pub trait WithSerdeDateTime {
        fn with_serde_datetime(&self) -> SerdeDateTime;
    }

    pub struct SerdeDateTime(DateTime<Utc>);

    impl WithSerdeDateTime for DateTime<Utc> {
        fn with_serde_datetime(&self) -> SerdeDateTime {
            SerdeDateTime(self.clone())
        }
    }

    impl serde::Serialize for SerdeDateTime {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serde_datetime::serialize(&self.0, serializer)
        }
    }

    impl<'de> serde::Deserialize<'de> for SerdeDateTime {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            serde_datetime::deserialize(deserializer).map(SerdeDateTime)
        }
    }

    #[test]
    fn test_serialize_datetime() {
        let dt = Utc.ymd(2025, 10, 2).and_hms(14, 30, 0);
        let json = serde_json::to_string(&dt.with_timezone(&Utc).with_serde_datetime()).unwrap();
        assert!(json.contains("2025-10-02 14:30:00 UTC"));
    }

    #[test]
    fn test_roundtrip_datetime() {
        let original = Utc.ymd(2025, 10, 2).and_hms(14, 30, 0);

        // Serialize
        let serialized = serde_json::to_string(&original.with_timezone(&Utc).with_serde_datetime()).unwrap();

        // Deserialize
        let deserialized: chrono::DateTime<Utc> =
            serde_json::from_str(&serialized).unwrap();

        assert_eq!(original, deserialized);
    }
}
