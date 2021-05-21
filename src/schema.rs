table! {
    use diesel::sql_types::*;
    use crate::logs::db::Logging_level;

    logs (id) {
        id -> Int4,
        message -> Text,
        level -> Logging_level,
        datetime -> Timestamp,
    }
}
