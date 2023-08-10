use std::io::Read;
use anyhow::Result;
use crate::parsio::ParsioClient;

mod parsio;

fn main() -> Result<()> {

    let parsio = ParsioClient::new();
    let docs = parsio.get_mailbox()?;

    println!("Got {} reservations", docs.len());
    for doc in docs {
        println!("{}", doc)
    }

    Ok(())
}
