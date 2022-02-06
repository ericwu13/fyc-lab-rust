use super::*;

pub trait Hashable {
    fn bytes (&self) -> Vec<u8>;

    // TODO: RISC-V Board (Sifive), design a hash-functions or ISA
    fn hash (&self) -> Hash {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}