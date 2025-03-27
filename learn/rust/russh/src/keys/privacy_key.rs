use russh::keys::{self, ssh_key::rand_core};
/// [russh::keys::PrivateKey]
pub struct PrivateKey {}
impl PrivateKey {
    /// [russh::keys::PrivateKey::random]
    pub fn random(
        rng: &mut impl rand_core::CryptoRngCore,
        algorithm: keys::Algorithm,
    ) -> Result<PrivateKey, keys::Error> {
        todo!()
    }
}
