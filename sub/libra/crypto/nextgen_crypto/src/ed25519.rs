// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

//! This module provides an API for the PureEdDSA signature scheme over the ed25519 twisted
//! Edwards curve as defined in [RFC8032](https://tools.ietf.org/html/rfc8032).
//!
//! Signature verification also checks and rejects non-canonical signatures.
//!
//! # Examples
//!
//! ```
//! use crypto::hash::{CryptoHasher, TestOnlyHasher};
//! use nextgen_crypto::{
//!     ed25519::*,
//!     traits::{Signature, SigningKey, Uniform},
//! };
//! use rand::{rngs::StdRng, SeedableRng};
//!
//! let mut hasher = TestOnlyHasher::default();
//! hasher.write("Test message".as_bytes());
//! let hashed_message = hasher.finish();
//!
//! let mut rng: StdRng = SeedableRng::from_seed([0; 32]);
//! let private_key = Ed25519PrivateKey::generate_for_testing(&mut rng);
//! let public_key: Ed25519PublicKey = (&private_key).into();
//! let signature = private_key.sign_message(&hashed_message);
//! assert!(signature.verify(&hashed_message, &public_key).is_ok());
//! ```
//! **Note**: The above example generates a private key using a private function intended only for
//! testing purposes. Production code should find an alternate means for secure key generation.

use crate::traits::*;
use core::convert::TryFrom;
use crypto::hash::HashValue;
use crypto_derive::{SilentDebug, SilentDisplay};
use curve25519_dalek::scalar::Scalar;
use ed25519_dalek;
use failure::prelude::*;
use serde::{Deserialize, Serialize};

/// An Ed25519 private key
#[derive(Serialize, Deserialize, SilentDisplay, SilentDebug)]
pub struct Ed25519PrivateKey(ed25519_dalek::SecretKey);

/// An Ed25519 public key
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Ed25519PublicKey(ed25519_dalek::PublicKey);

/// An Ed25519 signature
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Ed25519Signature(ed25519_dalek::Signature);

impl Ed25519PrivateKey {
    /// Serialize an Ed25519PrivateKey.
    pub fn to_bytes(&self) -> [u8; ed25519_dalek::SECRET_KEY_LENGTH] {
        self.0.to_bytes()
    }

    /// Deserialize an Ed25519PrivateKey without any validation checks apart from expected key size.
    fn from_bytes_unchecked(
        bytes: &[u8],
    ) -> std::result::Result<Ed25519PrivateKey, CryptoMaterialError> {
        match ed25519_dalek::SecretKey::from_bytes(bytes) {
            Ok(dalek_secret_key) => Ok(Ed25519PrivateKey(dalek_secret_key)),
            Err(_) => Err(CryptoMaterialError::DeserializationError),
        }
    }
}

impl Ed25519PublicKey {
    /// Serialize an Ed25519PublicKey.
    pub fn to_bytes(&self) -> [u8; ed25519_dalek::PUBLIC_KEY_LENGTH] {
        self.0.to_bytes()
    }

    /// Deserialize an Ed25519PublicKey without any validation checks apart from expected key size.
    pub(crate) fn from_bytes_unchecked(
        bytes: &[u8],
    ) -> std::result::Result<Ed25519PublicKey, CryptoMaterialError> {
        match ed25519_dalek::PublicKey::from_bytes(bytes) {
            Ok(dalek_public_key) => Ok(Ed25519PublicKey(dalek_public_key)),
            Err(_) => Err(CryptoMaterialError::DeserializationError),
        }
    }
}

impl Ed25519Signature {
    /// Serialize an Ed25519Signature.
    pub fn to_bytes(&self) -> [u8; ed25519_dalek::SIGNATURE_LENGTH] {
        self.0.to_bytes()
    }

    /// Deserialize an Ed25519Signature without any validation checks (malleability)
    /// apart from expected key size.
    pub(crate) fn from_bytes_unchecked(
        bytes: &[u8],
    ) -> std::result::Result<Ed25519Signature, CryptoMaterialError> {
        match ed25519_dalek::Signature::from_bytes(bytes) {
            Ok(dalek_signature) => Ok(Ed25519Signature(dalek_signature)),
            Err(_) => Err(CryptoMaterialError::DeserializationError),
        }
    }

    /// Check for correct size and malleability issues.
    /// This method ensures s is of canonical form and R does not lie on a small group.
    pub fn check_malleability(bytes: &[u8]) -> std::result::Result<(), CryptoMaterialError> {
        if bytes.len() != ed25519_dalek::SIGNATURE_LENGTH {
            return Err(CryptoMaterialError::WrongLengthError);
        }

        let mut s_bits: [u8; 32] = [0u8; 32];
        s_bits.copy_from_slice(&bytes[32..]);

        // Check if s is of canonical form.
        // We actually test if s < order_of_the_curve to capture malleable signatures.
        let s = Scalar::from_canonical_bytes(s_bits);
        if s == None {
            return Err(CryptoMaterialError::CanonicalRepresentationError);
        }

        // Check if the R lies on a small subgroup.
        // Even though the security implications of a small order R are unclear,
        // points of order <= 8 are rejected.
        let mut r_bits: [u8; 32] = [0u8; 32];
        r_bits.copy_from_slice(&bytes[..32]);

        let compressed = curve25519_dalek::edwards::CompressedEdwardsY(r_bits);
        let point = compressed.decompress();

        match point {
            Some(p) => {
                if p.is_small_order() {
                    Err(CryptoMaterialError::SmallSubgroupError)
                } else {
                    Ok(())
                }
            }
            None => Err(CryptoMaterialError::DeserializationError),
        }
    }
}

///////////////////////
// PrivateKey Traits //
///////////////////////

impl PrivateKey for Ed25519PrivateKey {
    type PublicKeyMaterial = Ed25519PublicKey;
}

impl SigningKey for Ed25519PrivateKey {
    type VerifyingKeyMaterial = Ed25519PublicKey;
    type SignatureMaterial = Ed25519Signature;

    fn sign_message(&self, message: &HashValue) -> Ed25519Signature {
        let secret_key: &ed25519_dalek::SecretKey = &self.0;
        let public_key: Ed25519PublicKey = self.into();
        let expanded_secret_key: ed25519_dalek::ExpandedSecretKey =
            ed25519_dalek::ExpandedSecretKey::from(secret_key);
        let sig = expanded_secret_key.sign(message.as_ref(), &public_key.0);
        Ed25519Signature(sig)
    }
}

impl Uniform for Ed25519PrivateKey {
    fn generate_for_testing<R>(rng: &mut R) -> Self
    where
        R: SeedableCryptoRng,
    {
        Ed25519PrivateKey(ed25519_dalek::SecretKey::generate(rng))
    }
}

impl PartialEq<Self> for Ed25519PrivateKey {
    fn eq(&self, other: &Self) -> bool {
        self.to_bytes() == other.to_bytes()
    }
}

impl Eq for Ed25519PrivateKey {}

// We could have a distinct kind of validation for the PrivateKey, for
// ex. checking the derived PublicKey is valid?
impl TryFrom<&[u8]> for Ed25519PrivateKey {
    type Error = CryptoMaterialError;

    /// Deserialize an Ed25519PrivateKey. This method will also check for key validity.
    fn try_from(bytes: &[u8]) -> std::result::Result<Ed25519PrivateKey, CryptoMaterialError> {
        // Note that the only requirement is that the size of the key is 32 bytes, something that
        // is already checked during deserialization of ed25519_dalek::SecretKey
        // Also, the underlying ed25519_dalek implementation ensures that the derived public key
        // is safe and it will not lie in a small-order group, thus no extra check for PublicKey
        // validation is required.
        Ed25519PrivateKey::from_bytes_unchecked(bytes)
    }
}
impl ValidKey for Ed25519PrivateKey {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}

impl Genesis for Ed25519PrivateKey {
    fn genesis() -> Self {
        let mut buf = [0u8; ed25519_dalek::SECRET_KEY_LENGTH];
        buf[ed25519_dalek::SECRET_KEY_LENGTH - 1] = 1;
        Self::try_from(buf.as_ref()).unwrap()
    }
}

//////////////////////
// PublicKey Traits //
//////////////////////

// Implementing From<&PrivateKey<...>> allows to derive a public key in a more elegant fashion
impl From<&Ed25519PrivateKey> for Ed25519PublicKey {
    fn from(secret_key: &Ed25519PrivateKey) -> Self {
        let secret: &ed25519_dalek::SecretKey = &secret_key.0;
        let public: ed25519_dalek::PublicKey = secret.into();
        Ed25519PublicKey(public)
    }
}

// We deduce PublicKey from this
impl PublicKey for Ed25519PublicKey {
    type PrivateKeyMaterial = Ed25519PrivateKey;
    fn length() -> usize {
        ed25519_dalek::PUBLIC_KEY_LENGTH
    }
}

impl std::hash::Hash for Ed25519PublicKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let encoded_pubkey = self.to_bytes();
        state.write(&encoded_pubkey);
    }
}

// Those are required by the implementation of hash above
impl PartialEq for Ed25519PublicKey {
    fn eq(&self, other: &Ed25519PublicKey) -> bool {
        self.to_bytes() == other.to_bytes()
    }
}

impl Eq for Ed25519PublicKey {}

// We deduce VerifyingKey from pointing to the signature material
// we get the ability to do `pubkey.validate(msg, signature)`
impl VerifyingKey for Ed25519PublicKey {
    type SigningKeyMaterial = Ed25519PrivateKey;
    type SignatureMaterial = Ed25519Signature;
}

impl std::fmt::Display for Ed25519PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(&self.0.to_bytes()[..]))
    }
}

impl TryFrom<&[u8]> for Ed25519PublicKey {
    type Error = CryptoMaterialError;

    /// Deserialize an Ed25519PublicKey. This method will also check for key validity, for instance
    ///  it will only deserialize keys that are safe against small subgroup attacks.
    fn try_from(bytes: &[u8]) -> std::result::Result<Ed25519PublicKey, CryptoMaterialError> {
        // We need to access the Edwards point which is not directly accessible from
        // ed25519_dalek::PublicKey, so we need to do some custom deserialization.
        if bytes.len() != ed25519_dalek::PUBLIC_KEY_LENGTH {
            return Err(CryptoMaterialError::WrongLengthError);
        }

        let mut bits = [0u8; ed25519_dalek::PUBLIC_KEY_LENGTH];
        bits.copy_from_slice(&bytes[..ed25519_dalek::PUBLIC_KEY_LENGTH]);

        let compressed = curve25519_dalek::edwards::CompressedEdwardsY(bits);
        let point = compressed
            .decompress()
            .ok_or(CryptoMaterialError::DeserializationError)?;

        // Check if the point lies on a small subgroup. This is required
        // when using curves with a small cofactor (in ed25519, cofactor = 8).
        if point.is_small_order() {
            return Err(CryptoMaterialError::SmallSubgroupError);
        }

        // Unfortunately, tuple struct `PublicKey` is private so we cannot
        // Ok(Ed25519PublicKey(ed25519_dalek::PublicKey(compressed, point)))
        // and we have to again invoke deserialization.
        Ed25519PublicKey::from_bytes_unchecked(bytes)
    }
}

impl ValidKey for Ed25519PublicKey {
    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }
}

//////////////////////
// Signature Traits //
//////////////////////

impl Signature for Ed25519Signature {
    type VerifyingKeyMaterial = Ed25519PublicKey;

    /// Checks that `self` is valid for `message` using `public_key`.
    fn verify(&self, message: &HashValue, public_key: &Ed25519PublicKey) -> Result<()> {
        self.verify_arbitrary_msg(message.as_ref(), public_key)
    }

    /// Checks that `self` is valid for an arbitrary &[u8] `message` using `public_key`.
    /// Outside of this crate, this particular function should only be used for native signature
    /// verification in move
    fn verify_arbitrary_msg(&self, message: &[u8], public_key: &Ed25519PublicKey) -> Result<()> {
        Ed25519Signature::check_malleability(&self.to_bytes())?;

        public_key
            .0
            .verify(message, &self.0)
            .map_err(std::convert::Into::into)
            .and(Ok(()))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }
}

impl std::hash::Hash for Ed25519Signature {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let encoded_pubkey = self.to_bytes();
        state.write(&encoded_pubkey);
    }
}

impl TryFrom<&[u8]> for Ed25519Signature {
    type Error = CryptoMaterialError;

    fn try_from(bytes: &[u8]) -> std::result::Result<Ed25519Signature, CryptoMaterialError> {
        Ed25519Signature::check_malleability(bytes)?;
        Ed25519Signature::from_bytes_unchecked(bytes)
    }
}

// Those are required by the implementation of hash above
impl PartialEq for Ed25519Signature {
    fn eq(&self, other: &Ed25519Signature) -> bool {
        self.to_bytes().as_ref() == other.to_bytes().as_ref()
    }
}

impl Eq for Ed25519Signature {}
