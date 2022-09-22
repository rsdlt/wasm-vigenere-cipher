use std::fmt::Display;

pub struct Hello {
    name: String,
}
pub fn new_hello() -> Hello {
    Hello {
        name: "Rodrigo".to_string(),
    }
}
impl Display for Hello {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
