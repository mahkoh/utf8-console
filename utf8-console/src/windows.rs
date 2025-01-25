use {
    std::io::Error,
    windows_sys::Win32::{
        Globalization::CP_UTF8,
        System::Console::{SetConsoleCP, SetConsoleOutputCP},
    },
};

pub(crate) fn enable() -> Result<(), Error> {
    let res = unsafe { SetConsoleCP(CP_UTF8) };
    if res == 0 {
        return Err(Error::last_os_error());
    }
    let res = unsafe { SetConsoleOutputCP(CP_UTF8) };
    if res == 0 {
        return Err(Error::last_os_error());
    }
    Ok(())
}
