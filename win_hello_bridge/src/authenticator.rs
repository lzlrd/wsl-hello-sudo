use crate::FailureReason;
use std::sync::mpsc;
use std::time::Duration;
use windows::{
    Security::{Credentials::KeyCredentialManager, Cryptography::CryptographicBuffer},
    Win32::UI::WindowsAndMessaging::{FindWindowW, SetForegroundWindow},
    UI::Popups::MessageDialog,
    core::{HSTRING, PCWSTR},
};

pub(crate) fn verify_user(key_name: &str, data_to_sign: &[u8]) -> Result<Vec<u8>, FailureReason> {
    if !KeyCredentialManager::IsSupportedAsync()?.get()? {
        let _ = MessageDialog::Create(&HSTRING::from("Windows Hello not supported"))?
            .ShowAsync()?
            .get();

        return Err(FailureReason::WindowsHelloNotSupported);
    }

    let key = {
        let result = KeyCredentialManager::OpenAsync(&HSTRING::from(key_name))?.get()?;
        FailureReason::from_credential_status(result.Status()?, key_name)?;
        result.Credential()?
    };

    let data = CryptographicBuffer::CreateFromByteArray(data_to_sign)?;

    let hello_focus = focus_hello_window();

    let result = key.RequestSignAsync(&data)?.get()?;

    drop(hello_focus);

    FailureReason::from_credential_status(result.Status()?, key_name)?;

    let buffer = result.Result()?;
    let mut out = windows::core::Array::<u8>::with_len(buffer.Length().unwrap() as usize);
    CryptographicBuffer::CopyToByteArray(&buffer, &mut out)?;

    Ok(out.to_vec())
}

fn focus_hello_window() -> mpsc::SyncSender<()> {
    let (send_shutdown, wait_for_shutdown) = mpsc::sync_channel(0);

    std::thread::spawn(move || {
        let hwnd = loop {
            let class_name = HSTRING::from("Credential Dialog Xaml Host");
            let hwnd = unsafe { FindWindowW(&class_name, PCWSTR::null()) };

            if hwnd.0 != 0 {
                break hwnd;
            }

            match wait_for_shutdown.recv_timeout(Duration::from_millis(500)) {
                Err(mpsc::RecvTimeoutError::Timeout) => continue,
                Err(mpsc::RecvTimeoutError::Disconnected) | Ok(()) => return,
            }
        };

        unsafe { SetForegroundWindow(hwnd) };
    });

    send_shutdown
}
