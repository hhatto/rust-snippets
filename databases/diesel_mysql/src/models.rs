#[derive(Queryable)]
pub struct Memo {
    pub id: i32,
    pub title: String,
    pub body: String,
}
