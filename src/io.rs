use std::io::{Error, ErrorKind, Read, Result, Write};

use crate::{Digest, Hasher};

/// A type which can be measured, returning a Digest.
pub trait Measurable {
    /// Return the measurment of this type.
    fn measure(&self) -> Digest;
}

impl Digest {
    /// Begins the process of verification.
    ///
    /// The supplied inner type will be measured and verified against the
    /// digest by the subsequent read operations.
    #[inline(always)]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "hasher", feature = "std"))))]
    pub fn verify<T>(self, inner: T) -> Verifier<T> {
        Verifier {
            measurer: self.hasher().measure(inner),
            digest: self,
        }
    }
}

impl Hasher {
    /// Begins the process of measuring a digest.
    ///
    /// The supplied inner type will be measured by the subsequent read/write
    /// operations.
    #[inline(always)]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "hasher", feature = "std"))))]
    pub fn measure<T>(self, inner: T) -> Measurer<T> {
        Measurer {
            hasher: self,
            inner,
        }
    }
}

/// A wrapper that computes the digest of data passing through it.
///
/// `Measurer<T>` wraps an inner type `T` and maintains a running digest of all data
/// that passes through read or write operations.
///
/// # Examples
///
/// ```rust
/// use oci_digest::{Hasher, Measurable};
/// use std::io::Read;
///
/// let data = b"hello world";
/// let digest = Hasher::default().chain(data).finish();
///
/// let mut reader = &data[..];
/// let mut measurer = Hasher::default().measure(&mut reader);
///
/// let mut buf = Vec::new();
/// measurer.read_to_end(&mut buf).unwrap();
/// assert_eq!(measurer.measure(), digest);
/// ```
#[derive(Clone, Debug)]
pub struct Measurer<T> {
    hasher: Hasher,
    inner: T,
}

impl<T> Measurer<T> {
    /// Returns the inner reader/writer.
    #[inline(always)]
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> Measurable for Measurer<T> {
    #[inline(always)]
    fn measure(&self) -> Digest {
        self.hasher.clone().finish()
    }
}

impl<T: Read> Read for Measurer<T> {
    #[inline(always)]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = self.inner.read(buf)?;
        self.hasher.update(&buf[..n]);
        Ok(n)
    }
}

impl<T: Write> Write for Measurer<T> {
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n = self.inner.write(buf)?;
        self.hasher.update(&buf[..n]);
        Ok(n)
    }

    #[inline(always)]
    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}

/// A wrapper that verifies streamed content matches an expected digest.
///
/// `Verifier<T>` wraps an inner type `T` and verifies that all data passing through
/// matches the digest. It will return an error if the computed digest doesn't
/// match at the end of the stream.
///
/// # Examples
///
/// ```rust
/// use std::io::Read;
/// # use oci_digest::{Digest, Hasher};
///
/// let data = b"hello world";
/// let digest = Hasher::default().chain(data).finish();
///
/// let mut reader = &data[..];
/// let mut verifier = digest.verify(&mut reader);
///
/// let mut buf = Vec::new();
/// verifier.read_to_end(&mut buf).unwrap(); // Verifies digest automatically
/// ```

#[derive(Clone, Debug)]
pub struct Verifier<T> {
    measurer: Measurer<T>,
    digest: Digest,
}

impl<T> Verifier<T> {
    /// Returns the inner reader.
    #[inline(always)]
    pub fn into_inner(self) -> T {
        self.measurer.into_inner()
    }
}

impl<T> Measurable for Verifier<T> {
    #[inline(always)]
    fn measure(&self) -> Digest {
        self.measurer.measure()
    }
}

impl<T: Read> Read for Verifier<T> {
    #[inline(always)]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        const MSG: &str = "digest mismatch";

        match self.measurer.read(buf)? {
            0 if self.measure() != self.digest => Err(Error::new(ErrorKind::InvalidData, MSG)),
            n => Ok(n),
        }
    }
}
