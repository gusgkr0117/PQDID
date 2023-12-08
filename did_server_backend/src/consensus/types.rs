//! Structure for the consensus protocol
use serde::{Deserialize, Serialize};

use crate::pqc_sign::types::{PublicKey, Signature};

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
    pub fn new(protocol_type: ProtocolType, data: Vec<u8>) -> Self {
        ProtocolPacket {
            protocol_type,
            data,
            protocol_id: None,
            signature: None,
        }
    }
}
