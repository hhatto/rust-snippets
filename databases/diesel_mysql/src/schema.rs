table! {
    memos (id) {
        id -> Unsigned<Integer>,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
