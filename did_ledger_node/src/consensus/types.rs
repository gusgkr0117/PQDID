//! Structure for the consensus protocol
use anyhow::Result;
use bitmaps::Bitmap;
use log::debug;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::pqc_sign::{
    types::{PublicKey, Signature},
    verify_sig,
};

use super::protocol::CONSENSUS_THRESHOLD;

#[derive(Serialize, Deserialize, Clone)]
pub enum ProtocolType {
    Read,
    Write,
    Ack,
}

#[derive(Serialize, Deserialize)]
pub struct ProtocolPacket {
    /// Type of the message(RW + Ack)
    pub protocol_type: ProtocolType,
    pub data: Vec<u8>,
    /// Public key used to identify the peers
    pub protocol_id: Option<PublicKey>,
    /// Signature used to verify the peers
    pub signature: Option<Signature>,
}

impl ProtocolPacket {
    pub fn verify(&self) -> Result<bool> {
        if let Some(sig) = self.signature.clone() {
            if let Some(pk) = self.protocol_id.clone() {
                let pkt_without_sig = ProtocolPacket {
                    protocol_type: self.protocol_type.clone(),
                    data: self.data.clone(),
                    protocol_id: self.protocol_id.clone(),
                    signature: None,
                };

                let raw_data = bincode::serialize(&pkt_without_sig).unwrap();

                debug!("ProtocolPacket verification start.. {}", hex::encode_upper(pk.value));
                return verify_sig(raw_data, sig, pk);
            }
        }
        Ok(false)
    }
}

#[derive(Serialize, Deserialize)]
pub struct AckPacket {
    pub protocol_type: ProtocolType,
    pub protocol_data: Vec<u8>,
    pub hash_value: u64,
}

pub struct CommitThresholder {
    pub time: SystemTime,
    pub bitmap: Bitmap<{ 3 * CONSENSUS_THRESHOLD + 1 }>,
    pub protocol_type: ProtocolType,
    pub protocol_data: Vec<u8>,
    pub done: bool,
}

impl CommitThresholder {
    pub fn new(protocol_type: ProtocolType, protocol_data: Vec<u8>) -> Self {
        let mut bitmap = Bitmap::<{ 3 * CONSENSUS_THRESHOLD + 1 }>::new();
        bitmap.set(0, true);
        CommitThresholder {
            time: SystemTime::now(),
            bitmap,
            protocol_type,
            protocol_data,
            done: false,
        }
    }
}
