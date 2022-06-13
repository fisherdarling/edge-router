pub struct Error;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
