use libc::wchar_t;
use failure::{Compat, Error};

#[derive(Debug, Fail)]
pub enum HidError {
    #[fail(display = "hidapi error: {}", message)]
    HidApiError { message: String },
    #[fail(
        display = "hidapi error: (could not get error message), caused by: {}",
        cause
    )]
    HidApiErrorEmptyWithCause {
        #[cause]
        cause: Compat<Error>,
    },
    #[fail(display = "hidapi error: (could not get error message)")]
    HidApiErrorEmpty,
    #[fail(display = "failed converting {:#X} to rust char", wide_char)]
    FromWideCharError { wide_char: wchar_t },
    #[fail(display = "Failed to initialize hidapi (maybe initialized before?)")]
    InitializationError,
    #[fail(display = "Failed opening hid device")]
    OpenHidDeviceError,
    #[fail(display = "Invalid data: size can not be 0")]
    InvalidZeroSizeData,
    #[fail(
        display = "Failed to send all data: only sent {} out of {} bytes",
        sent,
        all
    )]
    IncompleteSendError { sent: usize, all: usize },
    #[fail(display = "Can not set blocking mode to '{}'", mode)]
    SetBlockingModeError { mode: &'static str },
}
