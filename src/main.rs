use aws_sdk_dynamodb::{
    model::{AttributeValue, AttributeValueUpdate},
    Client, Error as OtherError, Region, PKG_VERSION,
};
use lambda_http::{service_fn, Body, Error, IntoResponse, Request, RequestExt, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::borrow::Cow;
use std::vec;
use url::{form_urlencoded::Parse, Url};
 
/// Main function
#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::from_env().region("us-west-2").load().await;
    let client = Client::new(&shared_config);

    lambda_http::run(service_fn(|request: Request| {
        println!("{:#?}", request);
        
        get_weather_stations(&client)
 
    }))
    .await?;

    Ok(())
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeatherStations {
    pub stations: Vec<WeatherStation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeatherStation {
    pub temperature: Option<u8>,
    pub lightlevel: Option<u8>,
    pub uvindex: Option<u8>,
    pub windspeed: Option<u8>,
    pub rainfall: Option<u8>,
    pub dateandtime: Option<String>,
    pub station_id: Option<String>,
}
 
async fn get_weather_stations(client: &Client) -> Result<impl IntoResponse, Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    // Query
    let req = client
        .query()
        .table_name("weatherstation_data")
        .key_condition_expression("station_id = :hashKey")
        .expression_attribute_values(":hashKey", AttributeValue::S("1".to_string()))
        .send()
        .await?;

    let mut weatherstations = WeatherStations { stations: vec![] };

    for list in req.items {
        for x in list {
            let mut ws = WeatherStation {
                temperature: None,
                lightlevel: None,
                uvindex: None,
                windspeed: None,
                rainfall: None,
                dateandtime: None,
                station_id: None,
            };
            for row in x {
                match &row.1 {
                    AttributeValue::S(val) => match &*row.0 {
                        "station_id" => {
                            ws.station_id = Some(val.to_string());
                        }
                        "dateandtime" => {
                            ws.dateandtime = Some(val.to_string());
                        }
                        "rainfall" => {
                            let x = val.parse::<u8>().unwrap();
                            ws.rainfall = Some(x);
                        }
                        "temperature" => {
                            let x = val.parse::<u8>().unwrap();
                            ws.temperature = Some(x);
                        }
                        "lightlevel" => {
                            let x = val.parse::<u8>().unwrap();
                            ws.lightlevel = Some(x);
                        }
                        "uvindex" => {
                            let x = val.parse::<u8>().unwrap();
                            ws.uvindex = Some(x);
                        }
                        "windspeed" => {
                            let x = val.parse::<u8>().unwrap();
                            ws.windspeed = Some(x);
                        }
                        _ => {}
                    },
                    AttributeValue::N(val) => match &*row.0 {
                        "station_id" => {
                            ws.station_id = Some(val.to_string());
                        }
                        "dateandtime" => {
                            ws.dateandtime = Some(val.to_string());
                        }
                        "rainfall" => {
                            let x = val.parse::<u8>().unwrap();
                            ws.rainfall = Some(x);
                        }
                        "temperature" => {
                            let x = val.parse::<u8>().unwrap();
                            ws.temperature = Some(x);
                        }
                        "lightlevel" => {
                            let x = val.parse::<u8>().unwrap();
                            ws.lightlevel = Some(x);
                        }
                        "uvindex" => {
                            let x = val.parse::<u8>().unwrap();
                            ws.uvindex = Some(x);
                        }
                        "windspeed" => {
                            let x = val.parse::<u8>().unwrap();
                            ws.windspeed = Some(x);
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            weatherstations.stations.push(ws);
        }
    }

    //dbg!(&weatherstations);
    let serialized_ws = serde_json::to_string(&weatherstations).unwrap();

    Ok(Response::builder().status(200).body(serialized_ws)?)
}
