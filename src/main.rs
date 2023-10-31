use anyhow::Result;
use embedded_svc::http::Method;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    http::server::{Configuration, EspHttpServer},
};
use log::info;
use routes::{handle_index, handle_set_color};
use std::{thread::sleep, time::Duration};
use wifi::connect_to_wifi;

mod routes;
mod template;
mod wifi;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let app_config = CONFIG;

    let peripherals = Peripherals::take()?;
    let sysloop = EspSystemEventLoop::take()?;

    let _wifi_connection = connect_to_wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    )?;

    let server_conf = Configuration::default();
    let mut server = EspHttpServer::new(&server_conf)?;
    server.fn_handler("/", Method::Get, handle_index)?;
    server.fn_handler("/set-color", Method::Get, handle_set_color)?;

    info!("Server awaiting connection");

    loop {
        sleep(Duration::from_millis(1000))
    }
}
