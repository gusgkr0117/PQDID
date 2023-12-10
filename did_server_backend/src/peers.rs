use std::time::Duration;

use anyhow::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::config::get_peers;

pub async fn send_to_peers(data: &Vec<u8>) -> Result<()> {
    let peer_list = get_peers()?;

    for peer_num in 0..peer_list.len() {
        let mut stream = match tokio::time::timeout(Duration::from_secs(1), TcpStream::connect(&peer_list[peer_num])).await {
            Ok(s) => s?,
            Err(_) => continue,
        };
        println!("Send tcp data to node {}", peer_num);
        stream.write_all(data).await?;
    }

    Ok(())
}

pub async fn check_peers() -> Result<Vec<bool>> {
    let peer_list = get_peers()?;
    let mut check_list = vec![false;4];
    
    for peer_num in 0..peer_list.len() {
        let _ = match tokio::time::timeout(
            Duration::from_secs(2),
            TcpStream::connect(&peer_list[peer_num])
        ).await {
          Ok(_) => {check_list[peer_num] = true}
          Err(_) => {check_list[peer_num] = false}
        };
    }
    Ok(check_list)
}

pub async fn send_to_peers_and_wait(data: &Vec<u8>) -> Result<Vec<Vec<u8>>> {
    let peer_list = get_peers()?;
    let mut response_list: Vec<Vec<u8>> = vec![vec![]; 4];

    for peer_num in 0..peer_list.len() {
        let mut stream = match tokio::time::timeout(Duration::from_secs(1), TcpStream::connect(&peer_list[peer_num])).await {
            Ok(s) => s?,
            Err(_) => continue,
        };
        stream.write_all(data).await?;
        stream.shutdown().await?;

        println!("Send tcp data to node {}", peer_num);
        stream.read_to_end(&mut response_list[peer_num]).await?;
    }

    Ok(response_list)
}