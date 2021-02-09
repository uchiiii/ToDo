table! {
    todos (id) {
        id -> Unsigned<Integer>,
        text -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        created_at -> Datetime,
    }
}

allow_tables_to_appear_in_same_query!(
    todos,
    users,
);
