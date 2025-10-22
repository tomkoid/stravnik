use clap::ValueEnum;
use serde::Serialize;

#[derive(Default, ValueEnum, Clone, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Service {
    Matrix,

    #[default]
    Ntfy,

    Discord,
}
