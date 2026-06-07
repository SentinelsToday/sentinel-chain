//! Borsh-encoded instructions the on-chain Sentinel attestation program accepts.

use borsh::{BorshDeserialize, BorshSerialize};

use crate::state::ClaimDigest;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub enum SentinelInstruction {
    /// Create a new attestation anchor PDA seeded by `(subject_did, claim_digest)`.
    AnchorAttestation {
        subject_did: String,
        claim_digest: ClaimDigest,
        issuer_pubkey: [u8; 32],
    },
    /// Close (and refund) an existing attestation anchor account.
    CloseAttestation { claim_digest: ClaimDigest },
}

impl SentinelInstruction {
    pub fn try_to_vec(&self) -> Result<Vec<u8>, std::io::Error> {
        borsh::to_vec(self)
    }

    pub fn try_from_slice(bytes: &[u8]) -> Result<Self, std::io::Error> {
        borsh::from_slice(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_both_variants() {
        let did = format!("did:sentinel:{}", "a".repeat(64));
        let digest = [7u8; 32];
        let anchor = SentinelInstruction::AnchorAttestation {
            subject_did: did.clone(),
            claim_digest: digest,
            issuer_pubkey: [3u8; 32],
        };
        let close = SentinelInstruction::CloseAttestation {
            claim_digest: digest,
        };

        for ix in [anchor, close] {
            let bytes = ix.try_to_vec().unwrap();
            let back = SentinelInstruction::try_from_slice(&bytes).unwrap();
            assert_eq!(ix, back);
        }
    }

    #[test]
    fn discriminant_is_first_byte() {
        let did = format!("did:sentinel:{}", "a".repeat(64));
        let anchor = SentinelInstruction::AnchorAttestation {
            subject_did: did,
            claim_digest: [0u8; 32],
            issuer_pubkey: [0u8; 32],
        };
        let close = SentinelInstruction::CloseAttestation {
            claim_digest: [0u8; 32],
        };
        let anchor_bytes = anchor.try_to_vec().unwrap();
        let close_bytes = close.try_to_vec().unwrap();
        assert_eq!(anchor_bytes[0], 0);
        assert_eq!(close_bytes[0], 1);
    }
}
