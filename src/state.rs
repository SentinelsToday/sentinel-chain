//! Account state stored on-chain by the Sentinel attestation program.

use borsh::{BorshDeserialize, BorshSerialize};

/// Maximum byte length of the DID stored inline.
pub const MAX_DID_LEN: usize = 96;

/// 32-byte sha256 digest of a canonical-JSON [`Claim`](sentinel_core::attestation::Claim).
pub type ClaimDigest = [u8; 32];

/// An on-chain anchor for a single attestation claim.
///
/// The PDA is derived as `["anchor", subject_did, claim_digest]`.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub struct AttestationAnchor {
    /// Layout version, bumped on incompatible changes.
    pub version: u8,
    /// `did:sentinel:<hex>` of the subject device.
    pub subject_did: String,
    /// SHA-256 over the canonical-JSON claim body.
    pub claim_digest: ClaimDigest,
    /// Unix timestamp (slot time) at which the anchor was created.
    pub anchored_at: i64,
    /// Public key of the issuer that signed the claim (Ed25519, 32 bytes).
    pub issuer_pubkey: [u8; 32],
}

impl AttestationAnchor {
    pub const CURRENT_VERSION: u8 = 1;

    pub fn new(
        subject_did: String,
        claim_digest: ClaimDigest,
        anchored_at: i64,
        issuer_pubkey: [u8; 32],
    ) -> Result<Self, StateError> {
        if subject_did.len() > MAX_DID_LEN {
            return Err(StateError::DidTooLong(subject_did.len()));
        }
        if !subject_did.starts_with("did:sentinel:") {
            return Err(StateError::InvalidDid);
        }
        Ok(Self {
            version: Self::CURRENT_VERSION,
            subject_did,
            claim_digest,
            anchored_at,
            issuer_pubkey,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("subject_did exceeds {MAX_DID_LEN} bytes (got {0})")]
    DidTooLong(usize),
    #[error("subject_did must start with `did:sentinel:`")]
    InvalidDid,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_digest() -> ClaimDigest {
        let mut d = [0u8; 32];
        d.iter_mut().enumerate().for_each(|(i, b)| *b = i as u8);
        d
    }

    #[test]
    fn round_trips_through_borsh() {
        let anchor = AttestationAnchor::new(
            format!("did:sentinel:{}", "a".repeat(64)),
            sample_digest(),
            1_700_000_000,
            [9u8; 32],
        )
        .unwrap();
        let bytes = borsh::to_vec(&anchor).unwrap();
        let back: AttestationAnchor = borsh::from_slice(&bytes).unwrap();
        assert_eq!(anchor, back);
        assert_eq!(back.version, AttestationAnchor::CURRENT_VERSION);
    }

    #[test]
    fn rejects_overlong_did() {
        let err = AttestationAnchor::new(
            "did:sentinel:".to_string() + &"x".repeat(MAX_DID_LEN),
            sample_digest(),
            0,
            [0u8; 32],
        )
        .unwrap_err();
        assert!(matches!(err, StateError::DidTooLong(_)));
    }

    #[test]
    fn rejects_bad_prefix() {
        let err = AttestationAnchor::new("did:foo:abc".into(), sample_digest(), 0, [0u8; 32])
            .unwrap_err();
        assert!(matches!(err, StateError::InvalidDid));
    }
}
