mod pqc_sign;
use anyhow::Result;
use hex::{FromHex, ToHex};
use pqc_sign::gen_keypair;

use crate::pqc_sign::types::PublicKey;

fn main() -> Result<()> {
    for i in 1..5 {
        let (pk, sk) = gen_keypair()?;
        println!("##### {}-th peer #####", i);
        println!("pk : {}", pk.encode_hex_upper::<String>());
        println!("sk : {}", sk.encode_hex_upper::<String>());
    }
    Ok(())
}
