use async_trait::async_trait;
use axum::{
    extract::{FromRequest, RequestParts},
    http::StatusCode,
    BoxError,
};
use axum_flash::{IncomingFlashes, Level};
use serde::*;

pub fn level_to_string(level: &Level) -> String {
    match level {
        Level::Debug => "debug".into(),
        Level::Info => "info".into(),
        Level::Success => "success".into(),
        Level::Warning => "warning".into(),
        Level::Error => "error".into(),
    }
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Flashes {
    pub store: Vec<(String, String)>,
    pub error_count: usize,
}

impl Flashes {
    #[allow(dead_code)]
    pub fn error(&mut self, msg: String) {
        self.store.push(("error".into(), msg));
    }

    #[allow(dead_code)]
    pub fn field_error(&mut self, mut msg: String) {
        msg.insert_str(0, "Field Error: ");
        self.store.push(("error".into(), msg));
    }

    #[allow(dead_code)]
    pub fn success(&mut self, msg: String) {
        self.store.push(("success".into(), msg));
    }

    #[allow(dead_code)]
    pub fn info(&mut self, msg: String) {
        self.store.push(("info".into(), msg));
    }

    #[allow(dead_code)]
    pub fn debug(&mut self, msg: String) {
        self.store.push(("debug".into(), msg));
    }

    #[allow(dead_code)]
    pub fn contains(&self, msg: String) -> bool {
        self.store.iter().any(|v| {
            v.0.contains("error") && v.1.to_lowercase().contains(&msg.to_lowercase().trim())
        })
    }

    #[allow(dead_code)]
    pub fn get_field_errors(&self, msg: String) -> Vec<(String, String)> {
        self.store
            .iter()
            .filter(|v| {
                v.0.contains("error") && v.1.to_lowercase().contains(&msg.to_lowercase().trim())
            })
            .cloned()
            .collect()
    }
}

#[async_trait]
impl<B> FromRequest<B> for Flashes
where
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let incoming_flashes = IncomingFlashes::from_request(req).await?;

        let store: Vec<(String, String)> = incoming_flashes
            .iter()
            .map(|(x, y)| (level_to_string(&x), y.to_string()))
            .collect();
        let error_count = store
            .iter()
            .filter(|x| x.0 == "error" && x.1.contains("Field Error:"))
            .count();

        Ok(Flashes { store, error_count })
    }
}
