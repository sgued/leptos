use std::{env::VarError, net::SocketAddr, str::FromStr};
use typed_builder::TypedBuilder;

/// This struct serves as a convenient place to store details used for rendering.
/// It's serialized into a file in the root called `.leptos.kdl` for cargo-leptos
/// to watch. It's also used in our actix and axum integrations to generate the
/// correct path for WASM, JS, and Websockets. Its goal is to be the single source
/// of truth for render options
#[derive(TypedBuilder, Clone)]
pub struct RenderOptions {
    /// The path and name of the WASM and JS files generated by wasm-bindgen
    /// For example, `/pkg/app` might be a valid input if your crate name was `app`.
    #[builder(setter(into))]
    pub pkg_path: String,
    /// Used to control whether the Websocket code for code watching is included.
    /// I recommend passing in the result of `env::var("RUST_ENV")`
    #[builder(setter(into), default)]
    pub environment: RustEnv,
    /// Provides a way to control the address leptos is served from.
    /// Using an env variable here would allow you to run the same code in dev and prod
    /// Defaults to `127.0.0.1:3000`
    #[builder(setter(into), default=SocketAddr::from(([127,0,0,1], 3000)))]
    pub socket_address: SocketAddr,
    /// The port the Websocket watcher listens on. Should match the `reload_port` in cargo-leptos(if using).
    /// Defaults to `3001`
    #[builder(default = 3001)]
    pub reload_port: u32,
}

impl RenderOptions {
    /// Creates a hidden file at ./.leptos_toml so cargo-leptos can monitor settings. We do not read from this file
    /// only write to it, you'll want to change the settings in your main function when you create RenderOptions
    pub fn write_to_file(&self) {
        use std::fs;
        let options = format!(
            r#"// This file is auto-generated. Changing it will have no effect on leptos. Change these by changing RenderOptions and rerunning
RenderOptions {{
    pkg-path "{}"
    environment "{:?}"
    socket-address "{:?}"
    reload-port {:?}
}}
"#,
            self.pkg_path, self.environment, self.socket_address, self.reload_port
        );
        fs::write("./.leptos.kdl", options).expect("Unable to write file");
    }
}
/// An enum that can be used to define the environment Leptos is running in. Can be passed to RenderOptions.
/// Setting this to the PROD variant will not include the websockets code for cargo-leptos' watch.
/// Defaults to PROD
#[derive(Debug, Clone)]
pub enum RustEnv {
    PROD,
    DEV,
}

impl Default for RustEnv {
    fn default() -> Self {
        Self::PROD
    }
}

impl FromStr for RustEnv {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let sanitized = input.to_lowercase();
        match sanitized.as_ref() {
            "dev" => Ok(Self::DEV),
            "development" => Ok(Self::DEV),
            "prod" => Ok(Self::PROD),
            "production" => Ok(Self::PROD),
            _ => Ok(Self::PROD),
        }
    }
}

impl From<&str> for RustEnv {
    fn from(str: &str) -> Self {
        let sanitized = str.to_lowercase();
        match sanitized.as_str() {
            "dev" => Self::DEV,
            "development" => Self::DEV,
            "prod" => Self::PROD,
            "production" => Self::PROD,
            _ => {
                panic!("Environment var is not recognized. Maybe try `dev` or `prod`")
            }
        }
    }
}
impl From<&Result<String, VarError>> for RustEnv {
    fn from(input: &Result<String, VarError>) -> Self {
        match input {
            Ok(str) => {
                let sanitized = str.to_lowercase();
                match sanitized.as_ref() {
                    "dev" => Self::DEV,
                    "development" => Self::DEV,
                    "prod" => Self::PROD,
                    "production" => Self::PROD,
                    _ => {
                        panic!("Environment var is not recognized. Maybe try `dev` or `prod`")
                    }
                }
            }
            Err(_) => Self::PROD,
        }
    }
}