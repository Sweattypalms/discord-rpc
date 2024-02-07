use config::ConfigError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    ConfigError(#[from] ConfigError),
    #[error(transparent)]
    TomlSe(#[from] toml::ser::Error),
    #[error("{0}")]
    BoxedStdError(#[from] Box<dyn std::error::Error>),
    #[error("{0}")]
    DiscordRpcError(#[from] discord_rich_presence::error::Error),
    #[error("{0}")]
    CtrlC(#[from] ctrlc::Error),
}