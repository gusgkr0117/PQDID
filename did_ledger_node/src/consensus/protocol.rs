//! Functions to progress the consensus protocol
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    future::Future,
    hash::{Hash, Hasher},
    sync::Arc,
};

use anyhow::{Context, Result};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use crate::{
    config::{get_pubkey, get_seckey},
    peers::{identify_peer, send_to_peers},
    pqc_sign::signing,
};

use super::{
    types::{AckPacket, CommitThresholder, ProtocolPacket, ProtocolType},
    Consensus,
};

use log::error;

pub const CONSENSUS_THRESHOLD: usize = 1;

fn hash_data(data: &Vec<u8>) -> u64 {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

impl Consensus {
    pub async fn run<Func, Fut>(self, f: Func) -> Result<()>
    where
        Func: FnOnce(ProtocolType, Vec<u8>) -> Fut + std::marker::Send + Copy + 'static,
        Fut: Future<Output = Result<Vec<u8>>> + std::marker::Send,
    {
        let listen = TcpListener::bind(self.local_addr).await?;
        loop {
            let (stream, _) = listen.accept().await?;
            let m_commit_queue = Arc::clone(&self.commit_queue);
            tokio::spawn(async move {
                if let Err(e) = process_stream(stream, m_commit_queue, f).await {
                    // Sinking all errors
                    error!("{:?}", e);
                };
            });
        }
    }
}

/// Broadcasting acknowledgement message to the peer nodes
/// * Signing the packet using its own secret key
async fn broadcast_ack(data_hash: Vec<u8>) -> Result<()> {
    let secret_key = get_seckey()?;
    let mut data_packet = ProtocolPacket {
        protocol_type: ProtocolType::Ack,
        data: data_hash,
        protocol_id: Some(get_pubkey()?),
        signature: None,
    };

    let raw_packet = bincode::serialize(&data_packet)?;
    data_packet.signature = Some(signing(raw_packet, secret_key)?);

    let final_packet = bincode::serialize(&data_packet)?;

    send_to_peers(&final_packet).await?;

    Ok(())
}

/// Processing the received packets
pub async fn process_stream<Func, Fut>(
    mut stream: TcpStream,
    commit_queue: Arc<Mutex<HashMap<u64, CommitThresholder>>>,
    callback: Func,
) -> Result<()>
where
    Func: FnOnce(ProtocolType, Vec<u8>) -> Fut,
    Fut: Future<Output = Result<Vec<u8>>>,
{
    let mut data: Vec<u8> = Vec::new();
    stream.read_to_end(&mut data).await?;

    let protocol_packet: ProtocolPacket = bincode::deserialize(data.as_slice())?;

    // Modifying the commit_queue
    match protocol_packet.protocol_type {
        // Send the corresponding document according to the received did
        ProtocolType::Read => {
            let response = callback(protocol_packet.protocol_type, protocol_packet.data).await?;
            stream.write_all(&bincode::serialize(&response)?).await?;
        }
        // Commit the CreateWallet
        ProtocolType::Write => {
            let mut queue = commit_queue.lock().await;
            let hash_value = hash_data(&protocol_packet.data);
            let commit_thresholder;

            if queue.contains_key(&hash_value) {
                // Update the existing committhresholder
                commit_thresholder = queue
                    .get_mut(&hash_value)
                    .expect("Unreachable : there is no corresponding value in hashmap");

                // If the transaction has already been executed, ignore the duplication
                if commit_thresholder.done {
                    return Ok(());
                }

                commit_thresholder.bitmap.set(0, true);
            } else {
                // Create a new committhresholder
                queue.insert(hash_value, CommitThresholder::new());
                broadcast_ack(bincode::serialize(&hash_value)?).await?;
                return Ok(());
            }

            broadcast_ack(bincode::serialize(&hash_value)?).await?;

            // Handling CommitThresholder
            if !commit_thresholder.done
                && commit_thresholder.bitmap.len() >= 2 * CONSENSUS_THRESHOLD + 1
            {
                commit_thresholder.done = true;
                let response =
                    callback(protocol_packet.protocol_type, protocol_packet.data).await?;
                stream.write_all(&bincode::serialize(&response)?).await?;
            }
        }
        ProtocolType::Ack => {
            // Verifying packet signature
            if protocol_packet.verify()? == false {
                return Ok(());
            }

            // Verifying the packet format
            let ack_pkt = bincode::deserialize::<AckPacket>(&protocol_packet.data)?;
            let pubkey = protocol_packet
                .protocol_id
                .context("no protocol id in packet")?;
            // Identifying the peer number
            let peer_num = identify_peer(pubkey)?.context("no matching peer")?;

            let mut queue = commit_queue.lock().await;
            let commit_thresholder;

            if queue.contains_key(&ack_pkt.hash_value) {
                // Update the existing committhresholder
                commit_thresholder = queue
                    .get_mut(&ack_pkt.hash_value)
                    .expect("Unreachable : there is no corresponding value in hashmap");
                commit_thresholder.bitmap.set(peer_num as usize, true);
            } else {
                // Create a new committhresholder
                queue.insert(ack_pkt.hash_value, CommitThresholder::new());
                return Ok(());
            }
        }
    };

    Ok(())
}
