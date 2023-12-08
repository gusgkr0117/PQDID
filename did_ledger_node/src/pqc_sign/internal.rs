//! FFI to bind the c library for sqisign
//! Reference : [Git repo of SQISign](https://github.com/SQISign/the-sqisign)
#[link(name = "sqisign")]
extern "C" {
    #[allow(dead_code)]
    pub fn sqisign_keypair(pk: *mut u8, sk: *mut u8) -> i32;
    pub fn sqisign_sign(
        sm: *mut u8,
        smlen: *mut u64,
        m: *const u8,
        mlen: u64,
        sk: *const u8,
    ) -> i32;
    #[allow(dead_code)]
    pub fn sqisign_open(
        m: *mut u8,
        mlen: *mut u64,
        sm: *const u8,
        smlen: u64,
        pk: *const u8,
    ) -> i32;
    pub fn sqisign_verify(
        m: *const u8,
        mlen: u64,
        sig: *const u8,
        siglen: u64,
        pk: *const u8,
    ) -> i32;
}
