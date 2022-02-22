pub struct Base {
    pub title: String,
    pub status: String,
}

impl Base {
    pub fn new(title: String, status: String) -> Self {
        Self { title, status }
    }
}
