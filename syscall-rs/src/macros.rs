#[macro_export]
macro_rules! parse_os_error {
    ($res:expr) => {{
        let code = $res;
        if code as isize == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(code)
        }
    }};
}