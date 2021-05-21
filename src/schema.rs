table! {
    logs (id) {
        id -> Int4,
        message -> Text,
        level -> Logging_level,
        datetime -> Timestamp,
    }
}
