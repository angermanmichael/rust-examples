use datafusion::error::Result;
use datafusion::prelude::*;
use std::env;

use env_logger::Env;
use log::{debug, error, info, trace, warn};

/// This example demonstrates executing a simple query against an Arrow data source (CSV) and
/// fetching results
#[tokio::main]
async fn main() -> Result<()> {
    // The `Env` lets us tweak what the environment
    // variables to read are and what the default
    // value is if they're missing
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    trace!("some trace log");
    debug!("some debug log");
    info!("some information log");
    warn!("some warning log");
    error!("some error log");

    // create local execution context
    let ctx = SessionContext::new();

    let datadir = env::current_dir().unwrap();

    //let testdata1 = String::from("/Users/ma/j/tmp06/rust-examples/datafusion/data/example.csv");
    let testdata = &format!("{}/data/red0.csv", datadir.display());

    // register csv file with the execution context
    ctx.register_csv("example", &testdata, CsvReadOptions::new())
        .await?;
    // execute the query
    let df = ctx.sql("Select a, f from example where f > 15").await?;
    // print the results
    df.show().await?;

    Ok(())
}
