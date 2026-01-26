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
        write!(f, "{self:?}")
    }
}

pub fn get_weather(location: &str, api_key: &str) -> Result<Weather> {
    //     .send()
    let resp = request(location, api_key).send()?;
    let weather = deserialize(&resp.text()?)?;
    Ok(weather)
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

fn request(location: &str, api_key: &str) -> RequestBuilder {
    reqwest::blocking::Client::new()
        .get("http://localhost:7878")
        .query(&[("query", &location), ("access_key", &api_key)])
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn request_builds_correct_request() {
        let req = request("London,UK", "dummy API key");
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
}
