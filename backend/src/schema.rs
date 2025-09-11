diesel::table! {
    user (id) {
        id -> Int4,
        user_name -> VarChar,
        password -> VarChar,
        email -> VarChar,
        notification_id -> VarChar,
    }
}

diesel::table! {
    product (id) {
        id -> Int4,
        image -> Text,
        title -> VarChar,
        content -> Text,
        price -> Float,
        currency -> VarChar,
    }
}