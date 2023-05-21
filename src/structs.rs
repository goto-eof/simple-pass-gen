#[derive(Debug)]
pub struct SimplePassGenError {
    pub message: String,
}
impl SimplePassGenError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
    }
}
