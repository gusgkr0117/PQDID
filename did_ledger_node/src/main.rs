mod config;
mod consensus;
mod database;
mod did_protocol;
mod peers;
mod pqc_sign;

use anyhow::Result;
use config::get_local_addr;
use did_protocol::protocol::did_run;

#[tokio::main]
async fn main() -> Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    let local_addr = get_local_addr()?;

    did_run(local_addr).await
}
