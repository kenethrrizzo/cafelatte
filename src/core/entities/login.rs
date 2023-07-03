use super::user::User;

pub struct Login {
    user: User,
    token: String,
}

impl Login {
    pub fn new() -> Self {
        Login {
            user: User::new(),
            token: "".to_string(),
        }
    }

    pub fn set_user(&mut self, user: User) {
        self.user = user;
    }

    pub fn set_token(&mut self, token: String) {
        self.token = token;
    }

    pub fn get_user(&self) -> User {
        self.user.clone()
    }

    pub fn get_token(&self) -> String {
        self.token.clone()
    }
}
