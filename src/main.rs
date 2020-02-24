use std::env;
use std::env::var_os;

use anyhow::Result;
use env_logger::DEFAULT_FILTER_ENV;
use lambda_runtime::lambda;
use log::info;

use crate::handlers::handler;
use crate::log_processing::process_log;
use crate::pipelines::compile_pipelines;
use crate::s3::open_s3_file;

mod error;
mod handlers;
mod log_processing;
mod output;
mod pipelines;
mod s3;
mod types;

mod config;

fn main() -> Result<()> {
    if env::var_os(DEFAULT_FILTER_ENV).is_none() {
        let log_level = format!("{}=info", env!("CARGO_PKG_NAME").replace("-", "_"));
        env::set_var(DEFAULT_FILTER_ENV, &log_level);
        env_logger::init();
        info!("Setting log level to {}", log_level);
    } else {
        env_logger::init();
    }

    //    rayon::ThreadPoolBuilder::new()
    //        .num_threads(THREADS_NUMBER)
    //        .build_global()?;
    //
    if var_os("INSIDE_LAMBDA").is_some() {
        lambda!(handler);
    } else {
        let config = config::from_args();
        let pipelines = compile_pipelines(&config);

        for bucket_key in &config.bucket_keys {
            process_log(
                open_s3_file(&config.bucket_name, &bucket_key, &config)?,
                &pipelines,
            )?
        }
    }

    Ok(())
}
