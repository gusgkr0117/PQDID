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

use log::{error, debug};

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
                    println!("{:?}", e);
                };
            });
        }
    }
}

/// Broadcasting acknowledgement message to the peer nodes
/// * Signing the packet using its own secret key
async fn broadcast_ack(ack_packet: AckPacket) -> Result<()> {
    let secret_key = get_seckey()?;
    let mut data_packet = ProtocolPacket {
        protocol_type: ProtocolType::Ack,
        data: bincode::serialize(&ack_packet)?,
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

    println!("protocol packet received");

    let protocol_packet: ProtocolPacket = bincode::deserialize(data.as_slice())?;

    // Modifying the commit_queue
    match protocol_packet.protocol_type {
        // Send the corresponding document according to the received did
        ProtocolType::Read => {
            println!("[Consensus] Read type message received");
            let response = match callback(protocol_packet.protocol_type, protocol_packet.data).await {
                Ok(v) => v,
                Err(_) => vec![0, 1, 2, 3],
            };
            stream.write_all(&bincode::serialize(&response)?).await?;
        }
        // Commit the CreateWallet
        ProtocolType::Write => {
            let mut queue = commit_queue.lock().await;
            let hash_value = hash_data(&protocol_packet.data);
            let commit_thresholder;

            println!("[Consensus] Write type message received");

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
                queue.insert(hash_value, CommitThresholder::new(protocol_packet.protocol_type.clone(), protocol_packet.data.clone()));
                let ack_packet = AckPacket {
                    protocol_type : protocol_packet.protocol_type.clone(),
                    protocol_data : protocol_packet.data.clone(),
                    hash_value,
                };

                broadcast_ack(ack_packet).await?;
                return Ok(());
            }

            let ack_packet = AckPacket {
                protocol_type : protocol_packet.protocol_type.clone(),
                protocol_data : protocol_packet.data.clone(),
                hash_value,
            };

            log::info!("Broadcasting ack message");
            broadcast_ack(ack_packet).await?;

            // Handling CommitThresholder
            if !commit_thresholder.done
                && commit_thresholder.bitmap.len() >= 2 * CONSENSUS_THRESHOLD + 1
            {
                commit_thresholder.done = true;
                callback(protocol_packet.protocol_type, protocol_packet.data).await?;
            }
        }
        ProtocolType::Ack => {
            // Verifying packet signature
            // if protocol_packet.verify()? == false {
            //     return Ok(());
            // }

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
                queue.insert(ack_pkt.hash_value, CommitThresholder::new(ack_pkt.protocol_type, ack_pkt.protocol_data));
                return Ok(());
            }

            // Handling CommitThresholder
            if !commit_thresholder.done
                && commit_thresholder.bitmap.len() >= 2 * CONSENSUS_THRESHOLD + 1
            {
                commit_thresholder.done = true;
                callback(commit_thresholder.protocol_type.clone(), commit_thresholder.protocol_data.clone()).await?;
            }
        }
    };

    Ok(())
}
