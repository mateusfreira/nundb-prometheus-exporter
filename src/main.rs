use hyper::{
    header::CONTENT_TYPE,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use prometheus::{Encoder, Gauge, TextEncoder};
use reqwest;
use std::env;

use lazy_static::lazy_static;
use prometheus::{labels, opts, register_gauge};
use core::str::Split;

lazy_static! {
    static ref NUNDB_OP_LOG_PEDDING_OPS: Gauge = register_gauge!(opts!(
        "nun_db_op_log_pending_ops",
        "Number pedding ops in oplog from primary to secoundaries",
        labels! {"databases" => "all",}
    ))
    .unwrap();

    static ref NUNDB_OP_LOG_FILE_SIZE: Gauge = register_gauge!(opts!(
        "nun_db_op_log_file_size",
        "Op log file size  in bytes",
        labels! {"databases" => "all",}
    ))
    .unwrap();

    static ref NUNDB_OP_LOG_OPS: Gauge = register_gauge!(opts!(
        "nun_db_op_log_OPS",
        "Count of oplog operations stored in the op log file",
        labels! {"databases" => "all",}
    ))
    .unwrap();

}
pub fn get_nun_db_user() -> String {
    match env::var_os("NUN_USER") {
        Some(user) => user.into_string().unwrap(),
        None => panic!("env NUN_USER is mandatory"),
    }
}

pub fn get_nun_db_pwd() -> String {
    match env::var_os("NUN_PWD") {
        Some(user) => user.into_string().unwrap(),
        None => panic!("env NUN_DB_PWD is mandatory"),
    }
}

pub fn get_nun_db_url() -> String {
    match env::var_os("NUN_URL") {
        Some(url) => url.into_string().unwrap(),
        None => panic!("env NUN_DB_URL is mandatory"),
    }
}
fn get_next_value(values: &mut Split<&str>) ->  f64 {
    let str_parts = values.next().unwrap();
    println!(
        "str_parts: {}",
        str_parts,
    );
    let mut parts = str_parts.trim().split(" ");
    parts.next(); //Key
    let str_value = parts.next().unwrap();
    str_value.parse::<f64>().unwrap()
}

async fn get_oplog_pending_ops() -> (f64, f64, f64) {
    let client = reqwest::Client::new();
    let res = client
        .post(get_nun_db_url())
        .body(format!(
            "auth {} {};oplog-state",
            get_nun_db_user(),
            get_nun_db_pwd()
        ))
        .send()
        .await;

    let text = res.unwrap().text().await.unwrap();
    let mut rep_parts = text.splitn(2, ";");
    rep_parts.next();
    let opps = rep_parts.next().unwrap();
    let mut parts = opps.splitn(2, " ");
    parts.next(); //Command oplog-state

    let mut values = parts.next().unwrap().split(",");

    let pedding_ops = get_next_value(&mut values);
    let op_log_file_size = get_next_value(&mut values);
    let op_log_count = get_next_value(&mut values);

    println!(
        "pedding_ops: {}, op_log_file_size: {}",
        pedding_ops, op_log_file_size
    );
    (
        pedding_ops,
        op_log_file_size,
        op_log_count,
    )
}

async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let encoder = TextEncoder::new();

    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    let (pedding_ops, op_log_file_size, op_log_count) =  get_oplog_pending_ops().await;

    NUNDB_OP_LOG_PEDDING_OPS.set(pedding_ops);
    NUNDB_OP_LOG_FILE_SIZE.set(op_log_file_size);
    NUNDB_OP_LOG_OPS.set(op_log_count);

    let response = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, encoder.format_type())
        .body(Body::from(buffer))
        .unwrap();

    Ok(response)
}

#[tokio::main]
async fn main() {
    let addr = ([0, 0, 0, 0], 9898).into();
    println!("Listening on http://{}", addr);

    let serve_future = Server::bind(&addr).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(serve_req))
    }));

    if let Err(err) = serve_future.await {
        eprintln!("server error: {}", err);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn get_connection_count() {
        let n = get_oplog_pending_ops().await;
        assert_eq!(n, 39.0);
    }
}
