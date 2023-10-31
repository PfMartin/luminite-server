use crate::template::templated;
use anyhow::Result;
use embedded_svc::http::server::{HandlerError, Request};
use esp_idf_svc::http::server::EspHttpConnection;
use url::Url;

pub fn handle_index(req: Request<&mut EspHttpConnection<'_>>) -> Result<(), HandlerError> {
    let mut res = req.into_ok_response()?;

    res.write(templated("Hello from luminite!").as_bytes())?;

    Ok(())
}

pub fn handle_set_color(r: Request<&mut EspHttpConnection<'_>>) -> Result<(), HandlerError> {
    let url = format!("http://base{}", r.uri());

    let params = Url::parse(&url)?
        .query_pairs()
        .map(|(_, value)| value.into())
        .collect::<Vec<String>>();

    let mut res = r.into_ok_response()?;

    res.write(
        templated(format!(
            "Color set - Red: {}, Green: {}, Blue: {}",
            params[0], params[1], params[2]
        ))
        .as_bytes(),
    )?;

    Ok(())
}
