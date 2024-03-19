use std::env;

#[derive(Debug)]
pub struct Env {
    pub libsql_url: String,
    pub libsql_auth_token: String,
}

pub fn env() -> Env {
    Env {
        libsql_url: env::var("LIBSQL_URL").expect("LIBSQL_URL must be set"),
        libsql_auth_token: env::var("LIBSQL_AUTH_TOKEN").expect("LIBSQL_AUTH_TOKEN must be set"),
    }
}
