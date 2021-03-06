#[derive(Serialize, Deserialize)]
pub struct Person {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}
