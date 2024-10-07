pub type UssdResult<T> = Result<T, UssdError>;

#[derive(Debug)]
pub enum UssdError {
    UnsupportedFormData { key: String, value: String },
}
