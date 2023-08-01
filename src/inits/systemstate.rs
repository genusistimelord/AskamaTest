use axum::extract::FromRef;

#[derive(Clone)]
pub struct SystemState {
    pub flash_config: axum_flash::Config,
    pub csrf: axum_csrf::CsrfConfig,
}

impl SystemState {
    pub fn new(flash_config: axum_flash::Config, csrf: axum_csrf::CsrfConfig) -> Self {
        Self { flash_config, csrf }
    }
}

impl FromRef<SystemState> for axum_flash::Config {
    fn from_ref(input: &SystemState) -> Self {
        input.flash_config.clone()
    }
}

impl FromRef<SystemState> for axum_csrf::CsrfConfig {
    fn from_ref(input: &SystemState) -> Self {
        input.csrf.clone()
    }
}
