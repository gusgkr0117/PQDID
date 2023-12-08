use crate::{consensus::types::ProtocolPacket, peers::send_to_peers};
use anyhow::{bail, Result};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use super::types::ProtocolType;

const CONSENSUS_THRESHOLD: usize = 1;

fn hash_data(data: &Vec<u8>) -> u64 {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

pub async fn send_consensus_packet(protocol_type: ProtocolType, data: Vec<u8>) -> Result<Vec<u8>> {
    let pkt = ProtocolPacket::new(protocol_type, data);
    let raw_packet_data = bincode::serialize(&pkt)?;
    let response_list = send_to_peers(&raw_packet_data).await?;
    let hash_list: Vec<u64> = response_list
        .clone()
        .into_iter()
        .map(|x| hash_data(&x))
        .collect();

    for i in 0..hash_list.len() {
        if hash_list.iter().filter(|&x| *x == hash_list[i]).count() >= 2 * CONSENSUS_THRESHOLD + 1 {
            return Ok(response_list[i].clone());
        }
    }

    bail!("No response")
}
