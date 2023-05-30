use crate::infrastructure::data::models::user::User as UserModel;

pub struct User {
    pub id: i32,
    pub name: String,
    pub surname: String,
}

impl User {
    pub fn from(user_model: UserModel) -> Self {
        User {
            id: user_model.id,
            name: user_model.name,
            surname: user_model.surname.unwrap_or_default(),
        }
    }
}
