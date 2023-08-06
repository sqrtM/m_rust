pub enum UserError {
    UsernameTaken,
    EmailTaken,
    UserNotFound,
    DuplicateEmail,
    FatalQueryError,
}

impl UserError {
    pub fn message(&self) -> String {
        match self {
            UserError::UsernameTaken => "This username is already taken!",
            UserError::EmailTaken => "This email is already taken!",
            UserError::UserNotFound => "Email or password is incorrect!",
            UserError::DuplicateEmail => "Oh no, multiple users with this email exist. Panicking...",
            UserError::FatalQueryError => "Internal error finding what you were looking for... Sorry!",
        }.to_string()
    }
}