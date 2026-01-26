use anyhow::{Context, Result};
use reqwest::blocking::RequestBuilder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;
use url::Host::Domain;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Weather {
    temperature: f64,
    summary: String,
}

impl Display for Weather {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:.1}ºC", self.summary, self.temperature)
    }
}

pub struct Weatherstack {
    api_key: String,
    pub base_url: String,
}

impl Weatherstack {
    #[must_use]
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_owned(),
            base_url: "http://localhost:7878/current".into(),
        }
    }

    pub fn get_weather(&self, location: &str) -> Result<Weather> {
        let resp = request(&self.base_url, location, &self.api_key).send()?;
        let weather = deserialize(&resp.text()?)?;
        Ok(weather)
    }
}

fn deserialize(text: &str) -> Result<Weather> {
    let val: Value = serde_json::from_str(text)?;
    let temperature = val
        .pointer("/current/temperature")
        .and_then(Value::as_f64)
        .with_context(|| format!("bad response: {val}"))?;
    let summary = val
        .pointer("/current/weather_descriptions/0")
        .and_then(Value::as_str)
        .with_context(|| format!("bad response: {val}"))?
        .to_string();
    Ok(Weather {
        temperature,
        summary,
    })
}

fn request(base_url: &str, location: &str, api_key: &str) -> RequestBuilder {
    reqwest::blocking::Client::new()
        .get(base_url)
        .query(&[("query", &location), ("access_key", &api_key)])
}

#[cfg(test)]
mod test {
    use std::fs;

    use http::StatusCode;
    use httpmock::{Method, MockServer};

    use super::*;

    #[test]
    fn request_builds_correct_request() {
        let ws = Weatherstack::new("dummy API key");
        let req = request(&ws.base_url, "London,UK", &ws.api_key);
        let req = req.build().unwrap();
        assert_eq!(req.method(), "GET", "wrong method");
        let url = req.url();
        assert_eq!(url.host(), Some(Domain("localhost")), "wrong host");
        let params: Vec<(_, _)> = url.query_pairs().collect();
        assert_eq!(
            params,
            vec![
                ("query".into(), "London,UK".into()),
                ("access_key".into(), "dummy API key".into())
            ],
            "wrong params"
        );
    }

    #[test]
    fn deserializa_creates_weather_result_from_body() {
        let req_body = fs::read_to_string("tests/data/weather.json").unwrap();
        let weather = deserialize(&req_body).unwrap();
        assert_eq!(
            weather,
            Weather {
                temperature: 11.2,
                summary: "Sunny".into(),
            },
            "wrong weather"
        );
    }

    #[test]
    fn get_weather_fn_makes_correct_api_call() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(Method::GET)
                .path("/current")
                .query_param("query", "London,UK")
                .query_param("access_key", "dummy API key");
            then.status(StatusCode::OK)
                .header("content-type", "application/json")
                .body_from_file("tests/data/weather.json");
        });
        let mut ws = Weatherstack::new("dummy API key");
        ws.base_url = server.base_url() + "/current";
        let weather = ws.get_weather("London,UK");
        mock.assert();
        assert_eq!(
            weather.unwrap(),
            Weather {
                temperature: 11.2,
                summary: "Sunny".into()
            },
            "wrong weather"
        );
    }
}
