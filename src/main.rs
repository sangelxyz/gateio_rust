mod utils {
    pub mod gateio;
    pub mod lib;
    pub mod serde;
}
use std::{format, println};
use utils::gateio::generate_signature;
use utils::lib::time_stamp;
use utils::serde::{Candles};

const SECRET: &str = "";
const KEY: &str = "";
const END_POINT_URL: &str = "https://api.gateio.ws";

#[tokio::main]
async fn main() {
    //    API call to candlesticks
    let response = get_api(
        "/api/v4/spot/candlesticks",
        Option::Some("currency_pair=btc_usdt"),
    )
    .await;

    // Response body.
    let response_body = response.text().await.unwrap();
    let sanitized_response = response_body.trim();

    let candles = serde_json::from_str::<Vec<Candles>>(sanitized_response);
    match candles {
        Ok(candles) => {
            // Successfully deserialized the response into a Vec<Candles>
            println!("{:#?}", candles);
        }
        Err(err) => {
            // Failed to deserialize the response
            println!("Error: {}", err);
        }
    }

    // List of Products
    let response = get_api("/api/v4/spot/currencies", Option::None).await;

    // Response body.
    let response_body = response.text().await.unwrap();
    let sanitized_response = response_body.trim();

    println!("{:?}", sanitized_response);

    // let products: Result<Vec<Products>, _> =
    //     serde_json::from_str(sanitized_response).map(|products: Vec<Vec<serde_json::Value>>| {
    //         println!("{:?}", products)
    //         // products
    //     .into_iter()
    //     .map(|values| Products {
    //         currency: as_string(&values[0]),
    //         delisted: as_string(&values[1]),
    //         withdraw_disabled: as_bool(&values[2]),
    //         withdraw_delayed: as_bool(&values[3]),
    //         deposit_disabled: as_bool(&values[4]),
    //         trade_disabled: as_bool(&values[5]),
    //         chain: as_string(&values[6]),
    //     })
    //     .collect()
    // });

    // match products {
    //     Ok(product) => {
    //         // Successfully deserialized the response into a Vec<Candles>
    //         println!("{:#?}", product);
    //     }
    //     Err(err) => {
    //         // Failed to deserialize the response
    //         println!("Error: {}", err);
    //     }
    // }
}
async fn get_api(end_point: &str, query: Option<&str>) -> reqwest::Response {
    // return query string or blank add ?
    let new_query = match query {
        Option::Some(qu) => format!("?{}", qu),
        Option::None => "".to_string(),
    };

    // Make a get request to gateio using our endpoint.
    let client = reqwest::Client::new();
    let url = format!("{}{}{}", END_POINT_URL, end_point, new_query);
    client
        .get(url)
        .header("Timestamp", time_stamp().to_string())
        // .header("message", "")
        .header("Key", KEY)
        .header(
            "Sign",
            generate_signature(
                "GET",
                "/api/v4/spot/candlesticks",
                "currency_pair=btc_usdt",
                SECRET,
            )
            .to_string(),
        )
        .send()
        .await
        .unwrap()
}
