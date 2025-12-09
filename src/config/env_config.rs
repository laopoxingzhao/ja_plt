use std::env;

fn load_env() {
    static ENV_LOADED: std::sync::Once = std::sync::Once::new();
    
    ENV_LOADED.call_once(|| {
        if dotenvy::from_filename("frontend/.env").is_err() {
            dotenvy::dotenv().ok();
        }
    });
}

pub fn database_url() -> String {
    load_env();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn jwt_secret() -> String {
    load_env();
    env::var("JWT_SECRET").unwrap_or_else(|_| "secret-development-key".to_string())
}