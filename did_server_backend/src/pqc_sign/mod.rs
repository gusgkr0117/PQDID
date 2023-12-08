//! Binding function to use sqisign c library
pub mod internal;
pub mod types;

use anyhow::{bail, Result};
use internal::sqisign_keypair;
use types::{PublicKey, SecretKey, Signature};

use self::{
    internal::{sqisign_sign, sqisign_verify},
    types::CRYPTO_BYTES,
};

/// Generate the sqisign key pair
#[allow(dead_code)]
pub fn gen_keypair() -> Result<(PublicKey, SecretKey)> {
    let mut pk = PublicKey::new();
    let mut sk = SecretKey::new();
    unsafe {
        if sqisign_keypair(pk.value.as_mut_ptr(), sk.value.as_mut_ptr()) != 0 {
            bail!("gen_keypair failed")
        }
    }
    Ok((pk, sk))
}

/// Generate a signature for the given message
pub fn signing(msg: Vec<u8>, sk: SecretKey) -> Result<Signature> {
    let mut sig: Signature = Signature::new();
    let mut sig_len: u64 = (CRYPTO_BYTES + 32) as u64;
    unsafe {
        if sqisign_sign(
            sig.value.as_mut_ptr(),
            &mut sig_len,
            msg.as_ptr(),
            msg.len() as u64,
            sk.value.as_ptr(),
        ) != 0
        {
            bail!("sign failed");
        }
    }
    Ok(sig)
}

/// Verify the given signature with the corresponding public key
pub fn verify_sig(msg: Vec<u8>, sig: Signature, pk: PublicKey) -> Result<bool> {
    unsafe {
        if sqisign_verify(
            msg.as_ptr(),
            msg.len() as u64,
            sig.value.as_ptr(),
            sig.value.len() as u64,
            pk.value.as_ptr(),
        ) != 0
        {
            return Ok(false);
        }
    }
    Ok(true)
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    #[test]
    fn sqisign_test() -> Result<()> {
        let msg = vec![1; 125];
        let wrong_msg = vec![0; 125];

        // Gererate a key pair
        let (pubkey, seckey) = gen_keypair()?;

        // Signing the message
        let sig = signing(msg.clone(), seckey)?;

        // Verifying the signature
        let verify_result = verify_sig(msg, sig.clone(), pubkey.clone())?;

        println!("verification result : {}", verify_result);

        // Verifying the signature with a wrong message
        let wrong_result = verify_sig(wrong_msg, sig, pubkey)?;
        println!("verification result(wrong) : {}", wrong_result);
        Ok(())
    }
}
