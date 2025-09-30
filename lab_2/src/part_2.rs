use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use url::Url;
use std::time::Duration;
use uuid::Uuid;
use humantime_serde;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RequestType {
    Success,
    Fail,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    #[serde(rename = "type")]
    pub request_type: RequestType,

    pub stream: Stream,
    pub gifts: Vec<Gifts>,
    pub debug: Debug,
}

#[derive(Debug, Deserialize, Serialize)]
struct Stream{
    user_id: Uuid,
    is_private: bool,
    settings: u32, 
    shard_url: Url,
    public_tariff: Public_tariff,
    private_tariff: Private_tariff
}

#[derive(Debug, Deserialize, Serialize)]
struct Public_tariff{
    id: u32,
    price: u32,

    #[serde(with = "humantime_serde")]
    duration: Duration,

    description: String
}

#[derive(Debug, Deserialize , Serialize)]
struct Private_tariff{
    client_price: u32,

    #[serde(with = "humantime_serde")]
    duration: Duration,

    description: String
}

#[derive(Debug, Deserialize, Serialize)]
struct Gifts{
    id: u32,
    price: u32,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Debug{
    #[serde(with = "humantime_serde")]
    duration: Duration,
    at: DateTime<Utc>
}

impl Request {
    pub fn read() -> Request{
        let json: Request = serde_json::from_str(json_data()).unwrap();
        let pretty = serde_json::to_string_pretty(&json).unwrap();
        //println!("{}", pretty);
        json
    }

    pub fn to_toml(){
      let request = Request::read();
      let toml = toml::to_string(&request).unwrap();
      println!("{}", toml);
    }
  }

fn json_data() -> &'static str{
r#"{
  "type": "success",
  "stream": {
    "user_id": "8d234120-0bda-49b2-b7e0-fbd3912f6cbf",
    "is_private": false,
    "settings": 45345,
    "shard_url": "https://n3.example.com/sapi",
    "public_tariff": {
      "id": 1,
      "price": 100,
      "duration": "1h",
      "description": "test public tariff"
    },
    "private_tariff": {
      "client_price": 250,
      "duration": "1m",
      "description": "test private tariff"
    }
  },
  "gifts": [{
    "id": 1,
    "price": 2,
    "description": "Gift 1"
  }, {
    "id": 2,
    "price": 3,
    "description": "Gift 2"
  }],
  "debug": {
    "duration": "234ms",
    "at": "2019-06-28T08:35:46+00:00"
  }
}"#
}
