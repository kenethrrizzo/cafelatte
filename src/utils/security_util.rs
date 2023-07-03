use bcrypt::{hash, verify};

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
///     salvo_skeleton::utils::security_util::verify_password(password, crypted_password));
/// ```
pub fn verify_password(password_to_verify: String, password: String) -> bool {
    verify(password_to_verify, &password).unwrap()
}
