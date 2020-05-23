table! {
    items (id) {
        id -> Uuid,
        demo_boolean -> Nullable<Bool>,
        demo_integer -> Nullable<Int4>,
        demo_json -> Nullable<Json>,
        demo_numeric -> Nullable<Numeric>,
        demo_text -> Nullable<Text>,
        demo_timestamp -> Nullable<Timestamp>,
    }
}
