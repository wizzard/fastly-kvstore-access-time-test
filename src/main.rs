use anyhow::Result;
use fastly::http::StatusCode;
use fastly::{KVStore, Request, Response, kv_store::KVStoreError, mime};
use std::time::Instant;

const KV_STORE_NAME: &str = "PXKVstoreTEST";
const TEST_HDR_KEY: &str = "x-test-enabled";
const TEST_HDR_VALUE: &str = "1";
const TEST_PATH: &str = "/test_kv";

fn read_kv_store(kv_store_name: &str) -> Result<(u128, u128)> {
    let start_time = Instant::now();
    let kv_store = match KVStore::open(kv_store_name) {
        Ok(kv) => kv.unwrap(),
        Err(e) => {
            return Err(e.into());
        }
    };
    let open_time = Instant::now().duration_since(start_time);
    let start_time = Instant::now();

    let short_string = match kv_store.lookup("short_string") {
        Ok(mut l) => l.take_body().into_string(),
        Err(KVStoreError::ItemNotFound) => "".to_string(),
        Err(_) => "".to_string(),
    };

    let u32_number = match kv_store.lookup("u32_number") {
        Ok(mut l) => l.take_body().into_string(),
        Err(KVStoreError::ItemNotFound) => "".to_string(),
        Err(_) => "".to_string(),
    };
    let u32_number = u32_number.parse::<u32>().unwrap_or(0);

    // INFO: if commenting the next 2 lookups, the test will still show high latency

    // 8KB string
    let long_string = match kv_store.lookup("long_string") {
        Ok(mut l) => l.take_body().into_string(),
        Err(KVStoreError::ItemNotFound) => "".to_string(),
        Err(_) => "".to_string(),
    };

    // 128KB JSON data (as a string)
    let json_data = match kv_store.lookup("json_data") {
        Ok(mut l) => l.take_body().into_string(),
        Err(KVStoreError::ItemNotFound) => "".to_string(),
        Err(_) => "".to_string(),
    };

    let read_time = Instant::now().duration_since(start_time);

    println!("short_string: {}", short_string);
    println!("u32_number: {}", u32_number);
    println!("long_string: {}", long_string);
    println!("json_data: {}", json_data);

    Ok((open_time.as_micros(), read_time.as_micros()))
}

#[fastly::main]
fn main(req: Request) -> Result<Response> {
    println!(
        "FASTLY_SERVICE_VERSION: {}",
        std::env::var("FASTLY_SERVICE_VERSION").unwrap_or_else(|_| String::new())
    );

    match req.get_path() {
        "/" => Ok(Response::from_status(StatusCode::OK)
            .with_content_type(mime::APPLICATION_JSON)
            .with_body("{}")),
        TEST_PATH => {
            if req
                .get_header_str(TEST_HDR_KEY)
                .unwrap_or_default()
                .contains(TEST_HDR_VALUE)
            {
                let times = match read_kv_store(KV_STORE_NAME) {
                    Ok(times) => times,
                    Err(_) => {
                        return Ok(Response::from_status(StatusCode::INTERNAL_SERVER_ERROR));
                    }
                };

                Ok(Response::from_status(StatusCode::OK)
                    .with_content_type(mime::APPLICATION_JSON)
                    .with_body(format!(
                        "{{\"open_time\": {}, \"read_time\": {}}}",
                        times.0, times.1
                    )))
            } else {
                Ok(Response::from_status(StatusCode::NOT_FOUND))
            }
        }
        _ => Ok(Response::from_status(StatusCode::NOT_FOUND)),
    }
}
