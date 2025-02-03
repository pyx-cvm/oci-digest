use sha2::{Digest as _, Sha256, Sha384, Sha512};

use crate::Digest;

impl Digest {
    /// Creates a new hasher instance based on the digest type.
    ///
    /// # Examples
    ///
    /// ```
    /// use oci_digest::{Digest, Hasher};
    ///
    /// let digest = Digest::Sha256([0u8; 32]);
    /// let hasher = digest.hasher();
    /// ```
    #[cfg_attr(docsrs, doc(cfg(feature = "hasher")))]
    pub fn hasher(&self) -> Hasher {
        match self {
            Self::Sha256(_) => Hasher::Sha256(Sha256::new()),
            Self::Sha384(_) => Hasher::Sha384(Sha384::new()),
            Self::Sha512(_) => Hasher::Sha512(Sha512::new()),
        }
    }
}

/// A unified hasher type supporting multiple hashing variants.
///
/// The `Hasher` enum provides a common interface for working with different SHA-2
/// hash implementations. It supports SHA-256, SHA-384, and SHA-512.
///
/// # Examples
///
/// ```
/// use oci_digest::Hasher;
///
/// let mut hasher = Hasher::default();  // Creates a SHA-256 hasher
/// hasher.update(b"Hello, world!");
/// let digest = hasher.finish();
/// ```
#[derive(Clone, Debug)]
pub enum Hasher {
    /// SHA-256 hasher variant
    Sha256(Sha256),

    /// SHA-384 hasher variant
    Sha384(Sha384),

    /// SHA-512 hasher variant
    Sha512(Sha512),
}

impl Default for Hasher {
    /// Returns a new hasher instance.
    ///
    /// The default hasher is undefined, but is practically a SHA-256 hasher.
    fn default() -> Self {
        Self::Sha256(Sha256::new())
    }
}

impl Hasher {
    /// Resets the hasher to its initial state.
    ///
    /// This clears any existing data in the hasher, allowing it to be reused.
    #[inline(always)]
    pub fn reset(&mut self) {
        match self {
            Self::Sha256(hasher) => hasher.reset(),
            Self::Sha384(hasher) => hasher.reset(),
            Self::Sha512(hasher) => hasher.reset(),
        }
    }

    /// Updates the hasher with new input data.
    ///
    /// # Arguments
    ///
    /// * `data` - A byte slice containing the data to hash
    ///
    /// # Examples
    ///
    /// ```
    /// use oci_digest::Hasher;
    ///
    /// let mut hasher = Hasher::default();
    /// hasher.update(b"Hello");
    /// hasher.update(b", world!");
    /// ```
    #[inline(always)]
    pub fn update(&mut self, data: &[u8]) {
        match self {
            Self::Sha256(hasher) => hasher.update(data),
            Self::Sha384(hasher) => hasher.update(data),
            Self::Sha512(hasher) => hasher.update(data),
        }
    }

    /// Chains data to the hasher and returns self for idiomatic API usage.
    ///
    /// # Arguments
    ///
    /// * `data` - A byte slice containing the data to hash
    ///
    /// # Examples
    ///
    /// ```
    /// use oci_digest::Hasher;
    ///
    /// let digest = Hasher::default()
    ///     .chain(b"Hello")
    ///     .chain(b", world!")
    ///     .finish();
    /// ```
    #[inline(always)]
    pub fn chain(mut self, data: &[u8]) -> Self {
        self.update(data);
        self
    }

    /// Consumes the hasher and returns the final digest.
    ///
    /// This method computes the final hash value and returns it as a `Digest`.
    /// After calling this method, the hasher cannot be used anymore.
    ///
    /// # Examples
    ///
    /// ```
    /// use oci_digest::Hasher;
    ///
    /// let digest = Hasher::default()
    ///     .chain(b"Hello, world!")
    ///     .finish();
    /// ```
    #[inline(always)]
    pub fn finish(self) -> Digest {
        match self {
            Self::Sha256(hasher) => Digest::Sha256(hasher.finalize().into()),
            Self::Sha384(hasher) => Digest::Sha384(hasher.finalize().into()),
            Self::Sha512(hasher) => Digest::Sha512(hasher.finalize().into()),
        }
    }
}
