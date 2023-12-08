use anyhow::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::config::get_peers;

pub async fn send_to_peers(data: &Vec<u8>) -> Result<Vec<Vec<u8>>> {
    let peer_list = get_peers()?;
    let mut response_list: Vec<Vec<u8>> = vec![vec![]; 4];

    for peer_num in 0..peer_list.len() {
        let mut stream = TcpStream::connect(&peer_list[peer_num]).await?;
        stream.write_all(data).await?;
        stream.read_to_end(&mut response_list[peer_num]).await?;
    }

    Ok(response_list)
}
