use std::{
    ffi::{c_void, OsStr},
    os::windows::ffi::OsStrExt,
};

use windows::{
    core::{Result, PCSTR, PCWSTR},
    Win32::{
        Foundation::{FARPROC, HINSTANCE},
        System::LibraryLoader::{GetProcAddress, LoadLibraryExW, LOAD_LIBRARY_FLAGS},
    },
};

pub(crate) struct Lib(HINSTANCE);

impl Lib {
    pub(crate) unsafe fn new<P: AsRef<OsStr>>(filename: P) -> Result<Lib> {
        let wide_filename: Vec<u16> = filename.as_ref().encode_wide().chain(Some(0)).collect();

        let instance = LoadLibraryExW(
            PCWSTR::from_raw(wide_filename.as_ptr()),
            None,
            LOAD_LIBRARY_FLAGS::default(),
        );

        instance.map(Lib)
    }

    /// Returns the function pointer of the given symbol name.
    ///
    /// # Satefy
    ///    1. The caller needs to ensure that the correct library was loaded.
    ///    2. The caller need to ensure that the correct symbol name was given.
    ///    3. The caller needs to ensure, that the function returns by the symbol
    ///       is compatible and sound with the given type `T`.
    ///    4. `symbol` must be null terminated.
    pub(crate) unsafe fn get<T>(&self, symbol: &[u8]) -> Option<&T> {
        if std::mem::size_of::<T>() != std::mem::size_of::<FARPROC>() {
            panic!("The given type for T is bigger than the size of FARPROC");
        }

        GetProcAddress(self.0, PCSTR(symbol.as_ptr())).map(|fn_pointer| {
            &*(&(fn_pointer as *const c_void) as *const *const c_void as *const T)
        })
    }
}
