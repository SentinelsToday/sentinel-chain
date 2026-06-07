//! On-chain Solana entrypoint. Only compiled when the `solana` feature is
//! enabled; pulled in automatically by `cargo build-sbf`.

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

use crate::instruction::SentinelInstruction;
use crate::state::AttestationAnchor;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ix = SentinelInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    match ix {
        SentinelInstruction::AnchorAttestation {
            subject_did,
            claim_digest,
            issuer_pubkey,
        } => {
            // Build the canonical on-chain state. Account allocation and PDA
            // creation are intentionally left to a follow-up commit (would
            // require system-program CPI + rent calculations); this entrypoint
            // currently just validates the payload encoding so off-chain
            // tooling can rely on it.
            let anchored_at = solana_program::clock::Clock::default().unix_timestamp;
            AttestationAnchor::new(subject_did, claim_digest, anchored_at, issuer_pubkey)
                .map_err(|_| ProgramError::InvalidArgument)?;
            msg!("sentinel-chain: anchor accepted");
            Ok(())
        }
        SentinelInstruction::CloseAttestation { .. } => {
            msg!("sentinel-chain: close accepted");
            Ok(())
        }
    }
}
