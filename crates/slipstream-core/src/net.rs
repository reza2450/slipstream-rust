use std::io::{Error, ErrorKind};

pub fn is_transient_udp_error(err: &Error) -> bool {
    match err.kind() {
        ErrorKind::WouldBlock | ErrorKind::TimedOut | ErrorKind::Interrupted => {
            return true;
        }
        _ => {}
    }

    match err.raw_os_error() {
        Some(code) if code == libc::ENETUNREACH || code == libc::EHOSTUNREACH => true,
        _ => false,
    }
}
