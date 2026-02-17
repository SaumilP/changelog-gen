use serde::Deserialize;

#[derive(Deserialize, Default, Debug, PartialEq)]
pub struct Config {
    pub project: Option<Project>,
    pub notifications: Option<Notifications>,
    pub telemetry: Option<Telemetry>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Project {
    pub name: String,
    pub repository: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Notifications {
    pub slack_webhook: Option<String>,
    pub discord_webhook: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Telemetry {
    pub enabled: bool,
}
