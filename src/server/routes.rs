use crate::led_strip::set_led_color;
use crate::server::template::templated;
use crate::CONFIG;
use anyhow::Result;
use embedded_svc::http::server::{HandlerError, Request};
use esp_idf_svc::http::server::EspHttpConnection;
use log::error;
use url::Url;

pub fn handle_index(req: Request<&mut EspHttpConnection<'_>>) -> Result<(), HandlerError> {
    let app_config = CONFIG;

    let mut res = req.into_ok_response()?;

    let _ = set_led_color(255, 150, 50, 1, app_config.led_strip_gpio)?;

    res.write(templated("Hello from luminite!").as_bytes())?;

    Ok(())
}

pub fn handle_set_color(req: Request<&mut EspHttpConnection<'_>>) -> Result<(), HandlerError> {
    let app_config = CONFIG;

    let params = parse_query_params(req.uri())?;

    let _ = set_led_color(
        params[0],
        params[1],
        params[2],
        1,
        app_config.led_strip_gpio,
    )?;

    let mut res = req.into_ok_response()?;

    res.write(
        templated(format!(
            "Color set - Red: {}, Green: {}, Blue: {}",
            params[0], params[1], params[2]
        ))
        .as_bytes(),
    )?;

    Ok(())
}

fn parse_query_params(uri: &str) -> Result<Vec<u8>> {
    let url = format!("http://base{}", uri);

    Ok(Url::parse(&url)?
        .query_pairs()
        .map(|(_, value)| match value.parse::<u8>() {
            Ok(c) => c,
            Err(err) => {
                error!("Error parsing query params: {}", err);
                0
            }
        })
        .collect::<Vec<u8>>())
}
