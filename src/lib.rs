//! Rust bindings for [DirectStorage](https://github.com/microsoft/DirectStorage).
//!
//! We try to provide the same abstraction level and coding style as [windows-rs](https://github.com/microsoft/windows-rs).
//!
//! For more documentation, please have a look at the header files of the official
//! distribution. We can't simply copy those because of licensing issues.
//!
//! This crate will panic if it can't find the shared libraries of DirectStorage.
//! Please refer to the README.md on how to install them.
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]

extern crate core;
#[cfg(not(target_os = "windows"))]
compile_error!("This crate is only supported on windows.");

use std::{
    ffi::{c_char, c_void},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign},
};

use load::Lib;
use once_cell::sync::Lazy;
use windows::{
    core::{Interface, Result, GUID, HRESULT},
    Win32::{
        Foundation::{BOOL, HANDLE, MAX_PATH},
        Graphics::Direct3D12::{
            ID3D12Device, ID3D12Fence, ID3D12Resource, D3D12_BOX, D3D12_TILED_RESOURCE_COORDINATE,
            D3D12_TILE_REGION_SIZE,
        },
    },
};

pub use crate::com_impl::*;

mod com_impl;
pub mod errors;
mod load;

// This is true for all currently supported Rust targets. Use upstream `c_size_t` once stable:
// https://github.com/rust-lang/rust/issues/88345
pub type c_size_t = usize;

pub const DSTORAGE_REQUEST_MAX_NAME: usize = 64;
pub const DSTORAGE_MIN_QUEUE_CAPACITY: u16 = 0x80;
pub const DSTORAGE_MAX_QUEUE_CAPACITY: u16 = 0x2000;
pub const DSTORAGE_DISABLE_BUILTIN_CPU_DECOMPRESSION: i32 = -1;

static DIRECT_STORAGE_LIB: Lazy<Libraries> = Lazy::new(|| {
    let ds = unsafe { Lib::new("dstorage.dll").expect("Can't load `dstorage.dll`") };
    let core = unsafe { Lib::new("dstoragecore.dll").expect("Can't load `dstoragecore.dll`") };
    Libraries { ds, _core: core }
});

struct Libraries {
    ds: Lib,
    _core: Lib,
}

pub unsafe fn DStorageSetConfiguration(configuration: &DSTORAGE_CONFIGURATION) -> Result<()> {
    let f: &unsafe extern "system" fn(configuration: *const DSTORAGE_CONFIGURATION) -> HRESULT =
        DIRECT_STORAGE_LIB
            .ds
            .get(b"DStorageSetConfiguration\0")
            .expect("Can't load function`DStorageSetConfiguration`");

    f(configuration as *const DSTORAGE_CONFIGURATION).ok()
}

pub unsafe fn DStorageGetFactory<T>() -> Result<T>
where
    T: Interface,
{
    let f: &unsafe extern "system" fn(riid: *const GUID, ppv: *mut *mut c_void) -> HRESULT =
        DIRECT_STORAGE_LIB
            .ds
            .get(b"DStorageGetFactory\0")
            .expect("Can't load function`DStorageGetFactory`");

    let mut result = None;
    f(&T::IID, &mut result as *mut _ as *mut _).and_some(result)
}

pub unsafe fn DStorageCreateCompressionCodec<T>(
    format: DSTORAGE_COMPRESSION_FORMAT,
    numThreads: u32,
) -> Result<T>
where
    T: Interface,
{
    let f: &unsafe extern "system" fn(
        format: DSTORAGE_COMPRESSION_FORMAT,
        numThreads: u32,
        riid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> HRESULT = DIRECT_STORAGE_LIB
        .ds
        .get(b"DStorageCreateCompressionCodec\0")
        .expect("Can't load function`DStorageCreateCompressionCodec`");

    let mut result = None;
    f(format, numThreads, &T::IID, &mut result as *mut _ as *mut _).and_some(result)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct DSTORAGE_PRIORITY(pub i8);

impl DSTORAGE_PRIORITY {
    pub const DSTORAGE_PRIORITY_LOW: Self = Self(-1);
    pub const DSTORAGE_PRIORITY_NORMAL: Self = Self(0);
    pub const DSTORAGE_PRIORITY_HIGH: Self = Self(1);
    pub const DSTORAGE_PRIORITY_REALTIME: Self = Self(2);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct DSTORAGE_REQUEST_SOURCE_TYPE(pub u64);

impl DSTORAGE_REQUEST_SOURCE_TYPE {
    pub const DSTORAGE_REQUEST_SOURCE_FILE: Self = Self(0);
    pub const DSTORAGE_REQUEST_SOURCE_MEMORY: Self = Self(1);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct DSTORAGE_REQUEST_DESTINATION_TYPE(pub u64);

impl DSTORAGE_REQUEST_DESTINATION_TYPE {
    pub const DSTORAGE_REQUEST_DESTINATION_MEMORY: Self = Self(0);
    pub const DSTORAGE_REQUEST_DESTINATION_BUFFER: Self = Self(1);
    pub const DSTORAGE_REQUEST_DESTINATION_TEXTURE_REGION: Self = Self(2);
    pub const DSTORAGE_REQUEST_DESTINATION_MULTIPLE_SUBRESOURCES: Self = Self(3);
    pub const DSTORAGE_REQUEST_DESTINATION_TILES: Self = Self(4);
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct DSTORAGE_QUEUE_DESC {
    pub SourceType: DSTORAGE_REQUEST_SOURCE_TYPE,
    pub Capacity: u16,
    pub Priority: DSTORAGE_PRIORITY,
    pub Name: *const c_char,
    pub Device: *mut ID3D12Device,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct DSTORAGE_QUEUE_INFO {
    pub Desc: DSTORAGE_QUEUE_DESC,
    pub EmptySlotCount: u16,
    pub RequestCountUntilAutoSubmit: u16,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct DSTORAGE_COMPRESSION_FORMAT(pub u8);

impl DSTORAGE_COMPRESSION_FORMAT {
    pub const DSTORAGE_COMPRESSION_FORMAT_NONE: Self = Self(0);
    pub const DSTORAGE_COMPRESSION_FORMAT_GDEFLATE: Self = Self(1);
    pub const DSTORAGE_CUSTOM_COMPRESSION_0: Self = Self(128);
}

// Since DirectStorage is compiled with MSVC, we have to use it's rules for C bitfields.
// MSVC will only pack fields of the same type in the same backing field.
//
// ```cpp
// struct DSTORAGE_REQUEST_OPTIONS {
//      DSTORAGE_COMPRESSION_FORMAT CompressionFormat : 8;     // uint8_t  -> saved into A
//      DSTORAGE_REQUEST_SOURCE_TYPE SourceType : 1;           // uint64_t -> packed together into B
//      DSTORAGE_REQUEST_DESTINATION_TYPE DestinationType : 7; // uint64_t -> packed together into B
//      UINT64 Reserved : 48;                                  // uint64_t -> packed together into B
// };
//
// // Resulting layout:
// struct Storage {
//      uint8_t A;
//      uint8_t PADDING[7];
//      uint64_t B;
// }
// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct DSTORAGE_REQUEST_OPTIONS {
    CompressionFormat: DSTORAGE_COMPRESSION_FORMAT,
    Bitfield: u64,
}

impl Default for DSTORAGE_REQUEST_OPTIONS {
    fn default() -> Self {
        Self {
            CompressionFormat: DSTORAGE_COMPRESSION_FORMAT::DSTORAGE_COMPRESSION_FORMAT_NONE,
            Bitfield: 0,
        }
    }
}

impl DSTORAGE_REQUEST_OPTIONS {
    pub fn CompressionFormat(&self) -> DSTORAGE_COMPRESSION_FORMAT {
        self.CompressionFormat
    }

    pub fn set_CompressionFormat(&mut self, value: DSTORAGE_COMPRESSION_FORMAT) {
        self.CompressionFormat = value;
    }

    pub fn SourceType(&self) -> DSTORAGE_REQUEST_SOURCE_TYPE {
        let size = u64::BITS;
        DSTORAGE_REQUEST_SOURCE_TYPE(self.Bitfield << (size - 1) >> (size - 1))
    }

    pub fn set_SourceType(&mut self, value: DSTORAGE_REQUEST_SOURCE_TYPE) {
        let mask = ((1 << 1) - 1) << 0;
        self.Bitfield &= !mask;
        self.Bitfield |= value.0 & mask;
    }

    pub fn DestinationType(&self) -> DSTORAGE_REQUEST_DESTINATION_TYPE {
        let size = u64::BITS;
        DSTORAGE_REQUEST_DESTINATION_TYPE(self.Bitfield << (size - 8) >> (size - 8 + 1))
    }

    pub fn set_DestinationType(&mut self, value: DSTORAGE_REQUEST_DESTINATION_TYPE) {
        let mask = ((1 << (8 - 1)) - 1) << 1;
        self.Bitfield &= !mask;
        self.Bitfield |= (value.0 << 1) & mask;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct DSTORAGE_DEBUG(pub u32);

impl DSTORAGE_DEBUG {
    pub const DSTORAGE_DEBUG_NONE: Self = Self(0x00);
    pub const DSTORAGE_DEBUG_SHOW_ERRORS: Self = Self(0x01);
    pub const DSTORAGE_DEBUG_BREAK_ON_ERROR: Self = Self(0x02);
    pub const DSTORAGE_DEBUG_RECORD_OBJECT_NAMES: Self = Self(0x04);
}

impl BitOr for DSTORAGE_DEBUG {
    type Output = DSTORAGE_DEBUG;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for DSTORAGE_DEBUG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for DSTORAGE_DEBUG {
    type Output = DSTORAGE_DEBUG;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for DSTORAGE_DEBUG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct DSTORAGE_SOURCE_FILE {
    pub Source: *mut IDStorageFile,
    pub Offset: u64,
    pub Size: u32,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct DSTORAGE_SOURCE_MEMORY {
    pub Source: *mut c_void,
    pub Size: u32,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct DSTORAGE_DESTINATION_MEMORY {
    pub Buffer: *mut c_void,
    pub Size: u32,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct DSTORAGE_DESTINATION_BUFFER {
    pub Resource: *mut ID3D12Resource,
    pub Offset: u64,
    pub Size: u32,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct DSTORAGE_DESTINATION_TEXTURE_REGION {
    pub Resource: *mut ID3D12Resource,
    pub SubresourceIndex: u32,
    pub Region: D3D12_BOX,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct DSTORAGE_DESTINATION_MULTIPLE_SUBRESOURCES {
    pub Resource: *mut ID3D12Resource,
    pub FirstSubresource: u32,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct DSTORAGE_DESTINATION_TILES {
    pub Resource: *mut ID3D12Resource,
    pub TiledRegionStartCoordinate: D3D12_TILED_RESOURCE_COORDINATE,
    pub TileRegionSize: D3D12_TILE_REGION_SIZE,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union DSTORAGE_SOURCE {
    pub Memory: DSTORAGE_SOURCE_MEMORY,
    pub File: DSTORAGE_SOURCE_FILE,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union DSTORAGE_DESTINATION {
    pub Memory: DSTORAGE_DESTINATION_MEMORY,
    pub Buffer: DSTORAGE_DESTINATION_BUFFER,
    pub Texture: DSTORAGE_DESTINATION_TEXTURE_REGION,
    pub MultipleSubresources: DSTORAGE_DESTINATION_MULTIPLE_SUBRESOURCES,
    pub Tiles: DSTORAGE_DESTINATION_TILES,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct DSTORAGE_REQUEST {
    pub Options: DSTORAGE_REQUEST_OPTIONS,
    pub Source: DSTORAGE_SOURCE,
    pub Destination: DSTORAGE_DESTINATION,
    pub UncompressedSize: u32,
    pub CancellationTag: u64,
    pub Name: *const c_char,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct DSTORAGE_COMMAND_TYPE(pub i32);

impl DSTORAGE_COMMAND_TYPE {
    pub const DSTORAGE_COMMAND_TYPE_NONE: Self = Self(-1);
    pub const DSTORAGE_COMMAND_TYPE_REQUEST: Self = Self(0);
    pub const DSTORAGE_COMMAND_TYPE_STATUS: Self = Self(1);
    pub const DSTORAGE_COMMAND_TYPE_SIGNAL: Self = Self(2);
    pub const DSTORAGE_COMMAND_TYPE_EVENT: Self = Self(3);
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct DSTORAGE_ERROR_PARAMETERS_REQUEST {
    pub Filename: [u16; MAX_PATH as usize],
    pub RequestName: [c_char; DSTORAGE_REQUEST_MAX_NAME],
    pub Request: DSTORAGE_REQUEST,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct DSTORAGE_ERROR_PARAMETERS_STATUS {
    pub StatusArray: *mut IDStorageStatusArray,
    pub Index: u32,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct DSTORAGE_ERROR_PARAMETERS_SIGNAL {
    pub Fence: *mut ID3D12Fence,
    pub Value: u64,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct DSTORAGE_ERROR_PARAMETERS_EVENT {
    pub Handle: HANDLE,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct DSTORAGE_ERROR_FIRST_FAILURE {
    pub HResult: HRESULT,
    pub CommandType: DSTORAGE_COMMAND_TYPE,
    pub Parameters: DSTORAGE_ERROR_PARAMETERS,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union DSTORAGE_ERROR_PARAMETERS {
    pub Request: DSTORAGE_ERROR_PARAMETERS_REQUEST,
    pub Status: DSTORAGE_ERROR_PARAMETERS_STATUS,
    pub Signal: DSTORAGE_ERROR_PARAMETERS_SIGNAL,
    pub Event: DSTORAGE_ERROR_PARAMETERS_EVENT,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct DSTORAGE_STAGING_BUFFER_SIZE(pub u32);

impl DSTORAGE_STAGING_BUFFER_SIZE {
    pub const DSTORAGE_STAGING_BUFFER_SIZE_0: Self = Self(0);
    pub const DSTORAGE_STAGING_BUFFER_SIZE_32MB: Self = Self(32 * 1024 * 1024);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct DSTORAGE_GET_REQUEST_FLAGS(pub u32);

impl DSTORAGE_GET_REQUEST_FLAGS {
    pub const DSTORAGE_GET_REQUEST_FLAG_SELECT_CUSTOM: Self = Self(0x01);
    pub const DSTORAGE_GET_REQUEST_FLAG_SELECT_BUILTIN: Self = Self(0x02);
    pub const DSTORAGE_GET_REQUEST_FLAG_SELECT_ALL: Self = Self(
        DSTORAGE_GET_REQUEST_FLAGS::DSTORAGE_GET_REQUEST_FLAG_SELECT_CUSTOM.0
            | DSTORAGE_GET_REQUEST_FLAGS::DSTORAGE_GET_REQUEST_FLAG_SELECT_BUILTIN.0,
    );
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct DSTORAGE_CUSTOM_DECOMPRESSION_FLAGS(pub u32);

impl DSTORAGE_CUSTOM_DECOMPRESSION_FLAGS {
    pub const DSTORAGE_CUSTOM_DECOMPRESSION_FLAG_NONE: Self = Self(0x00);
    pub const DSTORAGE_CUSTOM_DECOMPRESSION_FLAG_DEST_IN_UPLOAD_HEAP: Self = Self(0x01);
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct DSTORAGE_ERROR_RECORD {
    pub FailureCount: u32,
    pub FirstFailure: DSTORAGE_ERROR_FIRST_FAILURE,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST {
    pub Id: u64,
    pub CompressionFormat: DSTORAGE_COMPRESSION_FORMAT,
    pub Reserved: [u8; 3],
    pub Flags: DSTORAGE_CUSTOM_DECOMPRESSION_FLAGS,
    pub SrcSize: u64,
    pub SrcBuffer: *const c_void,
    pub DstSize: u64,
    pub DstBuffer: *const c_void,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct DSTORAGE_CUSTOM_DECOMPRESSION_RESULT {
    pub Id: u64,
    pub Result: HRESULT,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct DSTORAGE_CONFIGURATION {
    pub NumSubmitThreads: u32,
    pub NumBuiltInCpuDecompressionThreads: i32,
    pub ForceMappingLayer: BOOL,
    pub DisableBypassIO: BOOL,
    pub DisableTelemetry: BOOL,
    pub DisableGpuDecompressionMetacommand: BOOL,
    pub DisableGpuDecompression: BOOL,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct DSTORAGE_COMPRESSION(pub i32);

impl DSTORAGE_COMPRESSION {
    pub const DSTORAGE_COMPRESSION_FASTEST: Self = Self(-1);
    pub const DSTORAGE_COMPRESSION_DEFAULT: Self = Self(0);
    pub const DSTORAGE_COMPRESSION_BEST_RATIO: Self = Self(1);
}

#[cfg(test)]
mod tests {
    use std::mem::{align_of, size_of};

    use super::*;

    #[cfg(target_pointer_width = "32")]
    #[test]
    fn test_msvc_compat_32bit() {
        assert_eq!(size_of::<DSTORAGE_PRIORITY>(), 1);
        assert_eq!(align_of::<DSTORAGE_PRIORITY>(), 1);

        assert_eq!(size_of::<DSTORAGE_REQUEST_SOURCE_TYPE>(), 8);
        assert_eq!(align_of::<DSTORAGE_REQUEST_SOURCE_TYPE>(), 8);

        assert_eq!(size_of::<DSTORAGE_REQUEST_DESTINATION_TYPE>(), 8);
        assert_eq!(align_of::<DSTORAGE_REQUEST_DESTINATION_TYPE>(), 8);

        assert_eq!(size_of::<DSTORAGE_QUEUE_DESC>(), 24);
        assert_eq!(align_of::<DSTORAGE_QUEUE_DESC>(), 8);

        assert_eq!(size_of::<DSTORAGE_QUEUE_INFO>(), 32);
        assert_eq!(align_of::<DSTORAGE_QUEUE_INFO>(), 8);

        assert_eq!(size_of::<DSTORAGE_COMPRESSION_FORMAT>(), 1);
        assert_eq!(align_of::<DSTORAGE_COMPRESSION_FORMAT>(), 1);

        assert_eq!(size_of::<DSTORAGE_REQUEST_OPTIONS>(), 16);
        assert_eq!(align_of::<DSTORAGE_REQUEST_OPTIONS>(), 8);

        assert_eq!(size_of::<DSTORAGE_DEBUG>(), 4);
        assert_eq!(align_of::<DSTORAGE_DEBUG>(), 4);

        assert_eq!(size_of::<DSTORAGE_SOURCE_FILE>(), 24);
        assert_eq!(align_of::<DSTORAGE_SOURCE_FILE>(), 8);

        assert_eq!(size_of::<DSTORAGE_SOURCE_MEMORY>(), 8);
        assert_eq!(align_of::<DSTORAGE_SOURCE_MEMORY>(), 4);

        assert_eq!(size_of::<DSTORAGE_DESTINATION_MEMORY>(), 8);
        assert_eq!(align_of::<DSTORAGE_DESTINATION_MEMORY>(), 4);

        assert_eq!(size_of::<DSTORAGE_DESTINATION_BUFFER>(), 24);
        assert_eq!(align_of::<DSTORAGE_DESTINATION_BUFFER>(), 8);

        assert_eq!(size_of::<DSTORAGE_DESTINATION_TEXTURE_REGION>(), 32);
        assert_eq!(align_of::<DSTORAGE_DESTINATION_TEXTURE_REGION>(), 4);

        assert_eq!(size_of::<DSTORAGE_DESTINATION_MULTIPLE_SUBRESOURCES>(), 8);
        assert_eq!(align_of::<DSTORAGE_DESTINATION_MULTIPLE_SUBRESOURCES>(), 4);

        assert_eq!(size_of::<DSTORAGE_DESTINATION_TILES>(), 36);
        assert_eq!(align_of::<DSTORAGE_DESTINATION_TILES>(), 4);

        assert_eq!(size_of::<DSTORAGE_SOURCE>(), 24);
        assert_eq!(align_of::<DSTORAGE_SOURCE>(), 8);

        assert_eq!(size_of::<DSTORAGE_DESTINATION>(), 40);
        assert_eq!(align_of::<DSTORAGE_DESTINATION>(), 8);

        assert_eq!(size_of::<DSTORAGE_REQUEST>(), 104);
        assert_eq!(align_of::<DSTORAGE_REQUEST>(), 8);

        assert_eq!(size_of::<DSTORAGE_COMMAND_TYPE>(), 4);
        assert_eq!(align_of::<DSTORAGE_COMMAND_TYPE>(), 4);

        assert_eq!(size_of::<DSTORAGE_ERROR_PARAMETERS_REQUEST>(), 688);
        assert_eq!(align_of::<DSTORAGE_ERROR_PARAMETERS_REQUEST>(), 8);

        assert_eq!(size_of::<DSTORAGE_ERROR_PARAMETERS_STATUS>(), 8);
        assert_eq!(align_of::<DSTORAGE_ERROR_PARAMETERS_STATUS>(), 4);

        assert_eq!(size_of::<DSTORAGE_ERROR_PARAMETERS_SIGNAL>(), 16);
        assert_eq!(align_of::<DSTORAGE_ERROR_PARAMETERS_SIGNAL>(), 8);

        assert_eq!(size_of::<DSTORAGE_ERROR_PARAMETERS_EVENT>(), 4);
        assert_eq!(align_of::<DSTORAGE_ERROR_PARAMETERS_EVENT>(), 4);

        assert_eq!(size_of::<DSTORAGE_ERROR_FIRST_FAILURE>(), 696);
        assert_eq!(align_of::<DSTORAGE_ERROR_FIRST_FAILURE>(), 8);

        assert_eq!(size_of::<DSTORAGE_ERROR_RECORD>(), 704);
        assert_eq!(align_of::<DSTORAGE_ERROR_RECORD>(), 8);

        assert_eq!(size_of::<DSTORAGE_STAGING_BUFFER_SIZE>(), 4);
        assert_eq!(align_of::<DSTORAGE_STAGING_BUFFER_SIZE>(), 4);

        assert_eq!(size_of::<DSTORAGE_GET_REQUEST_FLAGS>(), 4);
        assert_eq!(align_of::<DSTORAGE_GET_REQUEST_FLAGS>(), 4);

        assert_eq!(size_of::<DSTORAGE_CUSTOM_DECOMPRESSION_FLAGS>(), 4);
        assert_eq!(align_of::<DSTORAGE_CUSTOM_DECOMPRESSION_FLAGS>(), 4);

        assert_eq!(size_of::<DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST>(), 48);
        assert_eq!(align_of::<DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST>(), 8);

        assert_eq!(size_of::<DSTORAGE_CUSTOM_DECOMPRESSION_RESULT>(), 16);
        assert_eq!(align_of::<DSTORAGE_CUSTOM_DECOMPRESSION_RESULT>(), 8);

        assert_eq!(size_of::<DSTORAGE_CONFIGURATION>(), 28);
        assert_eq!(align_of::<DSTORAGE_CONFIGURATION>(), 4);

        assert_eq!(size_of::<DSTORAGE_COMPRESSION>(), 4);
        assert_eq!(align_of::<DSTORAGE_COMPRESSION>(), 4);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn test_msvc_compat_64bit() {
        assert_eq!(size_of::<DSTORAGE_PRIORITY>(), 1);
        assert_eq!(align_of::<DSTORAGE_PRIORITY>(), 1);

        assert_eq!(size_of::<DSTORAGE_REQUEST_SOURCE_TYPE>(), 8);
        assert_eq!(align_of::<DSTORAGE_REQUEST_SOURCE_TYPE>(), 8);

        assert_eq!(size_of::<DSTORAGE_REQUEST_DESTINATION_TYPE>(), 8);
        assert_eq!(align_of::<DSTORAGE_REQUEST_DESTINATION_TYPE>(), 8);

        assert_eq!(size_of::<DSTORAGE_QUEUE_DESC>(), 32);
        assert_eq!(align_of::<DSTORAGE_QUEUE_DESC>(), 8);

        assert_eq!(size_of::<DSTORAGE_QUEUE_INFO>(), 40);
        assert_eq!(align_of::<DSTORAGE_QUEUE_INFO>(), 8);

        assert_eq!(size_of::<DSTORAGE_COMPRESSION_FORMAT>(), 1);
        assert_eq!(align_of::<DSTORAGE_COMPRESSION_FORMAT>(), 1);

        assert_eq!(size_of::<DSTORAGE_REQUEST_OPTIONS>(), 16);
        assert_eq!(align_of::<DSTORAGE_REQUEST_OPTIONS>(), 8);

        assert_eq!(size_of::<DSTORAGE_DEBUG>(), 4);
        assert_eq!(align_of::<DSTORAGE_DEBUG>(), 4);

        assert_eq!(size_of::<DSTORAGE_SOURCE_FILE>(), 24);
        assert_eq!(align_of::<DSTORAGE_SOURCE_FILE>(), 8);

        assert_eq!(size_of::<DSTORAGE_SOURCE_MEMORY>(), 16);
        assert_eq!(align_of::<DSTORAGE_SOURCE_MEMORY>(), 8);

        assert_eq!(size_of::<DSTORAGE_DESTINATION_MEMORY>(), 16);
        assert_eq!(align_of::<DSTORAGE_DESTINATION_MEMORY>(), 8);

        assert_eq!(size_of::<DSTORAGE_DESTINATION_BUFFER>(), 24);
        assert_eq!(align_of::<DSTORAGE_DESTINATION_BUFFER>(), 8);

        assert_eq!(size_of::<DSTORAGE_DESTINATION_TEXTURE_REGION>(), 40);
        assert_eq!(align_of::<DSTORAGE_DESTINATION_TEXTURE_REGION>(), 8);

        assert_eq!(size_of::<DSTORAGE_DESTINATION_MULTIPLE_SUBRESOURCES>(), 16);
        assert_eq!(align_of::<DSTORAGE_DESTINATION_MULTIPLE_SUBRESOURCES>(), 8);

        assert_eq!(size_of::<DSTORAGE_DESTINATION_TILES>(), 40);
        assert_eq!(align_of::<DSTORAGE_DESTINATION_TILES>(), 8);

        assert_eq!(size_of::<DSTORAGE_SOURCE>(), 24);
        assert_eq!(align_of::<DSTORAGE_SOURCE>(), 8);

        assert_eq!(size_of::<DSTORAGE_DESTINATION>(), 40);
        assert_eq!(align_of::<DSTORAGE_DESTINATION>(), 8);

        assert_eq!(size_of::<DSTORAGE_REQUEST>(), 104);
        assert_eq!(align_of::<DSTORAGE_REQUEST>(), 8);

        assert_eq!(size_of::<DSTORAGE_COMMAND_TYPE>(), 4);
        assert_eq!(align_of::<DSTORAGE_COMMAND_TYPE>(), 4);

        assert_eq!(size_of::<DSTORAGE_ERROR_PARAMETERS_REQUEST>(), 688);
        assert_eq!(align_of::<DSTORAGE_ERROR_PARAMETERS_REQUEST>(), 8);

        assert_eq!(size_of::<DSTORAGE_ERROR_PARAMETERS_STATUS>(), 16);
        assert_eq!(align_of::<DSTORAGE_ERROR_PARAMETERS_STATUS>(), 8);

        assert_eq!(size_of::<DSTORAGE_ERROR_PARAMETERS_SIGNAL>(), 16);
        assert_eq!(align_of::<DSTORAGE_ERROR_PARAMETERS_SIGNAL>(), 8);

        assert_eq!(size_of::<DSTORAGE_ERROR_PARAMETERS_EVENT>(), 8);
        assert_eq!(align_of::<DSTORAGE_ERROR_PARAMETERS_EVENT>(), 8);

        assert_eq!(size_of::<DSTORAGE_ERROR_FIRST_FAILURE>(), 696);
        assert_eq!(align_of::<DSTORAGE_ERROR_FIRST_FAILURE>(), 8);

        assert_eq!(size_of::<DSTORAGE_ERROR_RECORD>(), 704);
        assert_eq!(align_of::<DSTORAGE_ERROR_RECORD>(), 8);

        assert_eq!(size_of::<DSTORAGE_STAGING_BUFFER_SIZE>(), 4);
        assert_eq!(align_of::<DSTORAGE_STAGING_BUFFER_SIZE>(), 4);

        assert_eq!(size_of::<DSTORAGE_GET_REQUEST_FLAGS>(), 4);
        assert_eq!(align_of::<DSTORAGE_GET_REQUEST_FLAGS>(), 4);

        assert_eq!(size_of::<DSTORAGE_CUSTOM_DECOMPRESSION_FLAGS>(), 4);
        assert_eq!(align_of::<DSTORAGE_CUSTOM_DECOMPRESSION_FLAGS>(), 4);

        assert_eq!(size_of::<DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST>(), 48);
        assert_eq!(align_of::<DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST>(), 8);

        assert_eq!(size_of::<DSTORAGE_CUSTOM_DECOMPRESSION_RESULT>(), 16);
        assert_eq!(align_of::<DSTORAGE_CUSTOM_DECOMPRESSION_RESULT>(), 8);

        assert_eq!(size_of::<DSTORAGE_CONFIGURATION>(), 28);
        assert_eq!(align_of::<DSTORAGE_CONFIGURATION>(), 4);

        assert_eq!(size_of::<DSTORAGE_COMPRESSION>(), 4);
        assert_eq!(align_of::<DSTORAGE_COMPRESSION>(), 4);
    }

    #[test]
    fn test_bitfield() {
        let mut options = DSTORAGE_REQUEST_OPTIONS::default();
        options.set_CompressionFormat(
            DSTORAGE_COMPRESSION_FORMAT::DSTORAGE_COMPRESSION_FORMAT_GDEFLATE,
        );
        assert_eq!(
            options.CompressionFormat(),
            DSTORAGE_COMPRESSION_FORMAT::DSTORAGE_COMPRESSION_FORMAT_GDEFLATE
        );

        options.set_SourceType(DSTORAGE_REQUEST_SOURCE_TYPE::DSTORAGE_REQUEST_SOURCE_MEMORY);
        assert_eq!(
            options.SourceType(),
            DSTORAGE_REQUEST_SOURCE_TYPE::DSTORAGE_REQUEST_SOURCE_MEMORY
        );

        options.set_DestinationType(
            DSTORAGE_REQUEST_DESTINATION_TYPE::DSTORAGE_REQUEST_DESTINATION_TEXTURE_REGION,
        );
        assert_eq!(
            options.DestinationType(),
            DSTORAGE_REQUEST_DESTINATION_TYPE::DSTORAGE_REQUEST_DESTINATION_TEXTURE_REGION
        );

        options
            .set_CompressionFormat(DSTORAGE_COMPRESSION_FORMAT::DSTORAGE_COMPRESSION_FORMAT_NONE);
        options.set_SourceType(DSTORAGE_REQUEST_SOURCE_TYPE::DSTORAGE_REQUEST_SOURCE_FILE);
        options.set_DestinationType(
            DSTORAGE_REQUEST_DESTINATION_TYPE::DSTORAGE_REQUEST_DESTINATION_MEMORY,
        );

        assert_eq!(options, DSTORAGE_REQUEST_OPTIONS::default());
    }
}
