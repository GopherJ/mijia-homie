use eyre::Report;
use homie_device::DEFAULT_RECONNECT_INTERVAL;
use mijia::bluetooth::{MacAddress, ParseMacAddressError};
use rumqttc::MqttOptions;
use rustls::ClientConfig;
use serde::{Deserialize as _, Deserializer};
use serde_derive::Deserialize;
use stable_eyre::eyre::WrapErr;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::sync::Arc;
use std::time::Duration;

const DEFAULT_MQTT_PREFIX: &str = "homie";
const DEFAULT_DEVICE_ID: &str = "mijia-bridge";
const DEFAULT_DEVICE_NAME: &str = "Mijia bridge";
const DEFAULT_HOST: &str = "test.mosquitto.org";
const DEFAULT_PORT: u16 = 1883;
const DEFAULT_SENSOR_NAMES_FILENAME: &str = "sensor-names.toml";
const CONFIG_FILENAME: &str = "mijia-homie.toml";

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub mqtt: MqttConfig,
    pub homie: HomieConfig,
}

impl Config {
    pub fn from_file() -> Result<Config, Report> {
        Config::read(CONFIG_FILENAME)
    }

    fn read(filename: &str) -> Result<Config, Report> {
        let config_file =
            read_to_string(filename).wrap_err_with(|| format!("Reading {}", filename))?;
        Ok(toml::from_str(&config_file)?)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct MqttConfig {
    pub host: String,
    pub port: u16,
    pub use_tls: bool,
    pub username: Option<String>,
    pub password: Option<String>,
    pub client_name: Option<String>,
    #[serde(
        deserialize_with = "de_duration_seconds",
        rename = "reconnect_interval_seconds"
    )]
    pub reconnect_interval: Duration,
}

impl Default for MqttConfig {
    fn default() -> MqttConfig {
        MqttConfig {
            host: DEFAULT_HOST.to_owned(),
            port: DEFAULT_PORT,
            use_tls: false,
            username: None,
            password: None,
            client_name: None,
            reconnect_interval: DEFAULT_RECONNECT_INTERVAL,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct HomieConfig {
    pub device_id: String,
    pub device_name: String,
    pub prefix: String,
    pub sensor_names_filename: String,
    /// The minimum time to wait between sending consecutive readings for the same sensor.
    #[serde(
        deserialize_with = "de_duration_seconds",
        rename = "min_update_period_seconds"
    )]
    pub min_update_period: Duration,
}

fn de_duration_seconds<'de, D: Deserializer<'de>>(d: D) -> Result<Duration, D::Error> {
    let seconds = u64::deserialize(d)?;
    Ok(Duration::from_secs(seconds))
}

impl Default for HomieConfig {
    fn default() -> HomieConfig {
        HomieConfig {
            device_id: DEFAULT_DEVICE_ID.to_owned(),
            device_name: DEFAULT_DEVICE_NAME.to_owned(),
            prefix: DEFAULT_MQTT_PREFIX.to_owned(),
            sensor_names_filename: DEFAULT_SENSOR_NAMES_FILENAME.to_owned(),
            min_update_period: Duration::from_secs(0),
        }
    }
}

/// Construct the `MqttOptions` for connecting to the MQTT broker based on configuration options or
/// defaults.
pub fn get_mqtt_options(config: MqttConfig, device_id: &str) -> MqttOptions {
    let client_name = config.client_name.unwrap_or_else(|| device_id.to_owned());

    let mut mqtt_options = MqttOptions::new(client_name, config.host, config.port);

    mqtt_options.set_keep_alive(5);
    if let (Some(username), Some(password)) = (config.username, config.password) {
        mqtt_options.set_credentials(username, password);
    }

    if config.use_tls {
        let mut client_config = ClientConfig::new();
        client_config.root_store =
            rustls_native_certs::load_native_certs().expect("could not load platform certs");
        mqtt_options.set_tls_client_config(Arc::new(client_config));
    }
    mqtt_options
}

pub fn read_sensor_names(filename: &str) -> Result<HashMap<MacAddress, String>, Report> {
    let sensor_names_file =
        read_to_string(filename).wrap_err_with(|| format!("Reading {}", filename))?;
    let names = toml::from_str::<HashMap<String, String>>(&sensor_names_file)?
        .into_iter()
        .map(|(mac_address, name)| Ok::<_, ParseMacAddressError>((mac_address.parse()?, name)))
        .collect::<Result<HashMap<_, _>, _>>()?;
    Ok(names)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parsing the example config file should not give any errors.
    #[test]
    fn example_config() {
        Config::read("mijia-homie.example.toml").unwrap();
    }

    /// Parsing an empty config file should not give any errors.
    #[test]
    fn empty_config() {
        toml::from_str::<Config>("").unwrap();
    }
}