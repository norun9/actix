table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        body -> Text,
        created_at -> Nullable<Datetime>,
        updated_at -> Timestamp,
    }
}
