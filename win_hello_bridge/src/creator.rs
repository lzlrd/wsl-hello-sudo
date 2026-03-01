use crate::FailureReason;
use windows::{
    core::HSTRING,
    Security::{
        Credentials::{KeyCredentialCreationOption, KeyCredentialManager},
        Cryptography::CryptographicBuffer,
    },
};

pub(crate) fn create_public_key(key_name: &str) -> Result<String, FailureReason> {
    let public_key = {
        let result = KeyCredentialManager::RequestCreateAsync(
            &HSTRING::from(key_name),
            KeyCredentialCreationOption::FailIfExists,
        )?
        .get()?;

        match FailureReason::from_credential_status(result.Status()?, key_name) {
            Ok(()) => result
                .Credential()?
                .RetrievePublicKeyWithDefaultBlobType()?,
            Err(FailureReason::CredentialExists) => {
                let result = KeyCredentialManager::OpenAsync(&HSTRING::from(key_name))?.get()?;
                FailureReason::from_credential_status(result.Status()?, key_name)?;
                result
                    .Credential()?
                    .RetrievePublicKeyWithDefaultBlobType()?
            }
            Err(e) => return Err(e),
        }
    };

    Ok(format!(
        "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----\n",
        CryptographicBuffer::EncodeToBase64String(&public_key)?
    ))
}
