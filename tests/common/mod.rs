use curve25519_dalek::scalar::Scalar;
use rand_core::{CryptoRng, RngCore};

pub fn random_scalar<T: RngCore + CryptoRng>(rng: &mut T) -> Scalar {
    let mut wide_bytes = [0u8; 64];
    rng.fill_bytes(&mut wide_bytes);
    Scalar::from_bytes_mod_order_wide(&wide_bytes)
}
