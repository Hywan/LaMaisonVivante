use reqwest::{blocking::Client, Method};

#[derive(Debug)]
pub(crate) enum Error {
    UnableToSendWebThingRequest,
}

fn http_json(method: Method, url: &str, json_payload: &'static str) -> Result<(), Error> {
    let client = Client::new();

    match client
        .request(method, url)
        .header("Content-Type", "application/json")
        .body(json_payload)
        .send()
    {
        Ok(response) if response.status().is_success() => Ok(()),
        _ => Err(Error::UnableToSendWebThingRequest),
    }
}

pub(crate) fn close_blinds(blinds_url: &str) -> Result<(), Error> {
    let json_payload = "{\"close\": {}}";

    // Louise.
    http_json(
        Method::POST,
        &format!("{}/4/actions/close", blinds_url),
        json_payload,
    )?;
    // Éli.
    http_json(
        Method::POST,
        &format!("{}/3/actions/close", blinds_url),
        json_payload,
    )?;
    // Parents.
    http_json(
        Method::POST,
        &format!("{}/2/actions/close", blinds_url),
        json_payload,
    )?;

    Ok(())
}
