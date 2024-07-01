use std::env::VarError;

pub struct CorsHeaders {
    pub headers: String,
    pub methods: String,
    pub origin: String,
}

impl CorsHeaders {
    pub fn load_from_env() -> Result<Self, VarError> {
        Ok(Self {
            headers: std::env::var("Access_Control_Allow_Headers")?,
            methods: std::env::var("Access_Control_Allow_Methods")?,
            origin: std::env::var("Access_Control_Allow_Origin")?,
        })
    }
}
