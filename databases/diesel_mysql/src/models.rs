#[derive(Queryable)]
pub struct Memo {
    pub id: u32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
