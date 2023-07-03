use crate::infrastructure::data::models::user::User as UserModel;

#[derive(Clone, Debug)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub surname: String,
    pub phone_number: Option<String>,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new() -> User {
        User {
            id: None,
            name: "".to_string(),
            surname: "".to_string(),
            phone_number: None,
            email: "".to_string(),
            password: "".to_string(),
        }
    }

    pub fn from_user_model(row: UserModel) -> Self {
        User {
            id: row.id,
            name: row.name,
            surname: row.surname,
            phone_number: row.phone_number,
            email: row.email,
            password: row.password,
        }
    }

    pub fn from_user_model_vec(rows: Vec<UserModel>) -> Vec<Self> {
        let mut users: Vec<Self> = Vec::new();
        for row in &rows {
            users.push(Self::from_user_model(row.clone()));
        }

        users
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
}
