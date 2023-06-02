use crate::infrastructure::data::models::user::User as UserModel;

#[derive(Clone, Debug)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub surname: String,
}

impl User {
    pub fn from(user_model: UserModel) -> Self {
        User {
            id: Some(user_model.id),
            name: user_model.name,
            surname: user_model.surname.unwrap_or_default(),
        }
    }
}
