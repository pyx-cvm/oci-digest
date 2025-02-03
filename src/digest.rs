use core::{fmt::Debug, str::FromStr};

use super::Error;

/// A content-addressable digest following the OCI specification.
///
/// Represents a cryptographic hash in the format `<algorithm>:<hex>`.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Digest {
    /// a sha256 digest (i.e. `sha256:deadbeef...`)
    Sha256([u8; 32]),

    /// a sha384 digest (i.e. `sha384:deadbeef...`)
    Sha384([u8; 48]),

    /// a sha512 digest (i.e. `sha512:deadbeef...`)
    Sha512([u8; 64]),
}

impl Digest {
    pub(crate) fn algo(&self) -> &str {
        match self {
            Self::Sha256(_) => "sha256",
            Self::Sha384(_) => "sha384",
            Self::Sha512(_) => "sha512",
        }
    }
}

impl Debug for Digest {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Digest({})", self)
    }
}

impl AsRef<[u8]> for Digest {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Sha256(hash) => hash.as_ref(),
            Self::Sha384(hash) => hash.as_ref(),
            Self::Sha512(hash) => hash.as_ref(),
        }
    }
}

impl AsMut<[u8]> for Digest {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [u8] {
        match self {
            Self::Sha256(hash) => hash.as_mut(),
            Self::Sha384(hash) => hash.as_mut(),
            Self::Sha512(hash) => hash.as_mut(),
        }
    }
}

impl core::fmt::Display for Digest {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:", self.algo())?;
        for byte in self.as_ref() {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}

impl FromStr for Digest {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[inline(always)]
        fn dehex(byte: u8) -> Result<u8, Error> {
            Ok(match byte {
                b'0'..=b'9' => byte - b'0',
                b'a'..=b'f' => byte - b'a' + 10,
                _ => return Err(Error::Character),
            })
        }

        let (algo, hash) = s.split_once(':').unwrap_or(("sha256", s));

        let mut digest = match algo {
            "sha256" => Self::Sha256([0; 32]),
            "sha384" => Self::Sha384([0; 48]),
            "sha512" => Self::Sha512([0; 64]),
            _ => return Err(Error::Algorithm),
        };

        if hash.len() != digest.as_ref().len() * 2 {
            return Err(Error::Length);
        }

        let mut iter = hash.as_bytes().iter().copied();
        for b in digest.as_mut().iter_mut() {
            let hi = iter.next().ok_or(Error::Length)?;
            let lo = iter.next().ok_or(Error::Length)?;
            *b = dehex(hi)? << 4 | dehex(lo)?;
        }

        Ok(digest)
    }
}
