diesel::table! {
    users (id) {
        id -> Int4,
        user_name -> VarChar,
        password -> VarChar,
        email -> VarChar,
        notification_id -> VarChar,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        image -> Text,
        title -> VarChar,
        content -> Text,
        price -> Float,
        currency -> VarChar,
    }
}