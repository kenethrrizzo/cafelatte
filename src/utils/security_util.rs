use crate::core::{
    entities::user_payload::UserPayload,
    errors::{jwt_errors::JwtError, user_errors::UserError},
};
use bcrypt::{hash, verify};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::env;

pub fn create_jwt_token(payload: UserPayload) -> Result<String, JwtError> {
    match serde_json::to_value(payload) {
        Ok(payload) => {
            let secret = env::var("JWT_SECRET").unwrap();

            match encode(
                &Header::new(jsonwebtoken::Algorithm::HS256),
                &payload,
                &EncodingKey::from_secret(secret.as_ref()),
            ) {
                Ok(token) => Ok(token),
                Err(err) => {
                    log::error!("{}", err);

                    Err(JwtError::Unexpected)
                }
            }
        }
        Err(err) => {
            log::error!("{}", err);

            Err(JwtError::Unexpected)
        }
    }
}

pub fn verify_jwt_token(mut token: String) -> Result<UserPayload, JwtError> {
    let secret = env::var("JWT_SECRET").unwrap();

    match token.strip_prefix("Bearer ") {
        Some(t) => token = t.to_string(),
        None => {
            log::error!("Bearer not present.");

            return Err(JwtError::BearerNotPresent);
        }
    }

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

            Err(JwtError::Unexpected)
        }
    }
}

/// Función que retorna una contraseña encriptada.
///
/// Recibe un parámetro de tipo String que representa la contraseña del usuario, la encripta
/// con un tipo de costo por defecto y retorna un String que representa la contraseña encriptada.
///
/// ```
/// let result = cafelatte::utils::security_util::crypt_password(&"password".to_string()).unwrap();
/// assert_ne!(result, "password");
/// ```
pub fn crypt_password(password: &String) -> Result<String, UserError> {
    match hash(password, bcrypt::DEFAULT_COST) {
        Ok(crypted_password) => Ok(crypted_password),
        Err(err) => {
            log::error!("{}", err);
            Err(UserError::Unexpected)
        }
    }
}

/// Función que valida si una contraseña es válida.
///
/// Recibe dos parámetros de tipo String que representan la contraseña del usuario a verificar
/// y la contraseña almacaneada en la base de datos respectivamente. Usa la librería de bcrypt
/// para hacer la verificación y retorna un true en caso de que la contraseña sea válida.
///
/// ```
/// let password = "anything".to_string();
/// let crypted_password = cafelatte::utils::security_util::crypt_password(&password).unwrap();
/// assert_eq!(
///     true,
///     cafelatte::utils::security_util::verify_password(password, &crypted_password));
/// ```
pub fn verify_password(password_to_verify: String, password: &str) -> bool {
    match verify(password_to_verify, password) {
        Ok(_) => true,
        Err(err) => {
            log::error!("{}", err);
            false
        }
    }
}
