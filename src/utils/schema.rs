table! {
    card (id) {
        id -> Int4,
        user_id -> Int4,
        front -> Text,
        back -> Text,
        body -> Text,
        tags -> Array<Text>,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        username -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    card,
    users,
);
