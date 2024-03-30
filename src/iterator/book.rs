pub struct Book {
    name: String,
    author: String,
}
impl Book {
    pub fn new(name: &str, author: &str) -> Book {
        Book {
            name: name.to_string(),
            author: author.to_string(),
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_author(&self) -> String {
        self.author.clone()
    }
}
