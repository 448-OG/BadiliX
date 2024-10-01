use rand_chacha::ChaCha20Rng;
use rand_core::{RngCore, SeedableRng};

pub struct RandomBytes<const N: usize>([u8; N]);

impl<const N: usize> AsRef<[u8]> for RandomBytes<N> {
    fn as_ref(&self) -> &[u8] {
        self.expose_borrowed()
    }
}

impl<const N: usize> RandomBytes<N> {
    pub fn gen() -> Self {
        let mut rng = ChaCha20Rng::from_entropy();
        let mut buffer = [0u8; N];
        rng.fill_bytes(&mut buffer);

        let outcome = RandomBytes(buffer);

        buffer.fill(0);

        outcome
    }

    /// Clone the data. Be careful with this as it retains the secret in memory.
    /// It is recommended to call `Csprng::zeroize()` after consuming this in order to zeroize the memory
    pub fn expose(&self) -> [u8; N] {
        self.0
    }

    /// Clone the data. Be careful with this as it retains the secret in memory.
    /// It is recommended to call `Csprng::zeroize()` after consuming this in order to zeroize the memory
    pub fn expose_borrowed(&self) -> &[u8] {
        self.0.as_ref()
    }

    /// Get the inner value of the struct. This is only available in a debug build and
    /// is enforced by the flag `#[cfg(debug_assertions)]`
    #[cfg(debug_assertions)]
    pub fn dangerous_debug(&self) -> &[u8; N] {
        &self.0
    }
}

impl<const N: usize> core::fmt::Debug for RandomBytes<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CsprngArray(REDACTED)").finish()
    }
}

impl<const N: usize> core::fmt::Display for RandomBytes<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CsprngArray(REDACTED)").finish()
    }
}
