use crate::core::{entities::user_payload::UserPayload, errors::user_errors::UserError};
use bcrypt::{hash, verify};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::env;

pub fn create_jwt_token(payload: UserPayload) -> String {
    let payload = serde_json::to_value(payload).unwrap();
    let secret = env::var("JWT_SECRET").unwrap();
    let token = encode(
        &Header::new(jsonwebtoken::Algorithm::HS256),
        &payload,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();

    token
}

pub fn verify_jwt_token(mut token: String) -> Result<UserPayload, UserError> {
    if let Some(t) = token.strip_prefix("Bearer ") {
        token = t.to_string();
    } else {
        log::error!("Bearer not present.");
        return Err(UserError::Unexpected);
    }

    let secret = env::var("JWT_SECRET").unwrap();

    match decode(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    ) {
        Ok(decoded_token) => {
            let payload: serde_json::Value = decoded_token.claims;
            let user_payload: UserPayload = serde_json::from_value(payload).unwrap();

            Ok(user_payload)
        }
        Err(err) => {
            log::error!("{}", err);
            Err(UserError::Unexpected)
        }
    }
}

/// Función que retorna una contraseña encriptada.
///
/// Recibe un parámetro de tipo String que representa la contraseña del usuario, la encripta
/// con un tipo de costo por defecto y retorna un String que representa la contraseña encriptada.
///
/// ```
/// let result = salvo_skeleton::utils::security_util::crypt_password(&"password".to_string());
/// assert_ne!(result, "password");
/// ```
pub fn crypt_password(password: &String) -> String {
    hash(password, bcrypt::DEFAULT_COST).unwrap()
}

/// Función que valida si una contraseña es válida.
///
/// Recibe dos parámetros de tipo String que representan la contraseña del usuario a verificar
/// y la contraseña almacaneada en la base de datos respectivamente. Usa la librería de bcrypt
/// para hacer la verificación y retorna un true en caso de que la contraseña sea válida.
///
/// ```
/// let password = "anything".to_string();
/// let crypted_password = salvo_skeleton::utils::security_util::crypt_password(&password);
/// assert_eq!(
///     true,
///     salvo_skeleton::utils::security_util::verify_password(password, &crypted_password));
/// ```
pub fn verify_password(password_to_verify: String, password: &String) -> bool {
    verify(password_to_verify, &password).unwrap()
}
