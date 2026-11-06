//! Cryptographic checksum generation, verification, and executable integrity diagnostics.

use crate::errors::DistributionError;
use crate::model::{InstallationManifest, VerificationStatus};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Computes the hex-encoded SHA-256 digest of a file on disk.
pub fn compute_file_sha256(path: &Path) -> Result<String, DistributionError> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

/// Verifies that a file's SHA-256 digest matches the expected hex string.
pub fn verify_file_checksum(path: &Path, expected_sha256: &str) -> Result<(), DistributionError> {
    let actual = compute_file_sha256(path)?;
    if !actual.eq_ignore_ascii_case(expected_sha256.trim()) {
        return Err(DistributionError::ChecksumMismatch {
            path: path.to_path_buf(),
            expected: expected_sha256.to_string(),
            actual,
        });
    }
    Ok(())
}

/// Verifier checking the integrity of installed AEK files against an installation manifest.
#[derive(Debug, Default)]
pub struct InstallationVerifier;

impl InstallationVerifier {
    /// Audits an installation manifest and verifies each managed file exists on disk.
    #[must_use]
    pub fn verify_installation(manifest: &InstallationManifest) -> VerificationStatus {
        if !manifest.binary_path.is_file() {
            return VerificationStatus::Mismatch;
        }

        for file_path in &manifest.managed_files {
            if !file_path.exists() {
                return VerificationStatus::Mismatch;
            }
        }

        VerificationStatus::Verified
    }
}
