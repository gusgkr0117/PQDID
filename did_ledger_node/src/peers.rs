use crate::{
    config::{get_peer_keys, get_peers},
    pqc_sign::types::PublicKey,
};
use anyhow::Result;
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn send_to_peers(data: &Vec<u8>) -> Result<bool> {
    let peer_list = get_peers()?;

    for peer in peer_list {
        let mut stream = TcpStream::connect(peer).await?;
        stream.write_all(data).await?;
    }

    Ok(true)
}

pub fn identify_peer(pubkey: PublicKey) -> Result<Option<u8>> {
    let peer_pub_list: Vec<PublicKey> = get_peer_keys()?;
    for peer_num in 0..peer_pub_list.len() {
        if peer_pub_list[peer_num] == pubkey {
            return Ok(Some((peer_num + 1) as u8));
        }
    }
    Ok(None)
}
