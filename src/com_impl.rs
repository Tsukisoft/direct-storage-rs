use std::{
    ffi::c_void,
    mem::{transmute, MaybeUninit},
};

use windows::{
    core::{
        IUnknown, IUnknownImpl, IUnknown_Vtbl, Interface, Result, RuntimeName, Vtable, GUID,
        HRESULT, PCSTR, PCWSTR,
    },
    Win32::{
        Foundation::HANDLE, Graphics::Direct3D12::ID3D12Fence,
        Storage::FileSystem::BY_HANDLE_FILE_INFORMATION,
    },
};

use crate::{
    c_size_t, DSTORAGE_COMPRESSION, DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
    DSTORAGE_CUSTOM_DECOMPRESSION_RESULT, DSTORAGE_DEBUG, DSTORAGE_ERROR_RECORD,
    DSTORAGE_GET_REQUEST_FLAGS, DSTORAGE_QUEUE_DESC, DSTORAGE_QUEUE_INFO, DSTORAGE_REQUEST,
};

#[repr(transparent)]
pub struct IDStorageFile(IUnknown);

impl IDStorageFile {
    pub unsafe fn Close(&self) {
        (Vtable::vtable(self).Close)(Vtable::as_raw(self))
    }
    pub unsafe fn GetFileInformation(&self, info: *mut BY_HANDLE_FILE_INFORMATION) -> Result<()> {
        (Vtable::vtable(self).GetFileInformation)(Vtable::as_raw(self), info).ok()
    }
}

unsafe impl Vtable for IDStorageFile {
    type Vtable = IDStorageFile_Vtbl;
}

unsafe impl Interface for IDStorageFile {
    const IID: GUID = GUID {
        data1: 0x5DE95E7B,
        data2: 0x955A,
        data3: 0x4868,
        data4: [0xA7, 0x3C, 0x24, 0x3B, 0x29, 0xF4, 0xB8, 0xDA],
    };
}

impl RuntimeName for IDStorageFile {}

#[allow(non_camel_case_types)]
pub trait IDStorageFile_Impl: Sized {
    unsafe fn Close(&self);
    unsafe fn GetFileInformation(&self, info: *mut BY_HANDLE_FILE_INFORMATION) -> HRESULT;
}

#[repr(C)]
#[doc(hidden)]
pub struct IDStorageFile_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub Close: unsafe extern "system" fn(this: *mut c_void),
    pub GetFileInformation: unsafe extern "system" fn(
        this: *mut c_void,
        info: *mut BY_HANDLE_FILE_INFORMATION,
    ) -> HRESULT,
}

impl IDStorageFile_Vtbl {
    pub const fn new<
        Identity: IUnknownImpl<Impl = Impl>,
        Impl: IDStorageFile_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn Close<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageFile_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close()
        }
        unsafe extern "system" fn GetFileInformation<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageFile_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
            info: *mut BY_HANDLE_FILE_INFORMATION,
        ) -> HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFileInformation(info)
        }
        // TODO ask upstream to make the "IUnknown_Vtbl::new" to based on the "implement" feature.
        Self {
            base__: IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Close: Close::<Identity, Impl, OFFSET>,
            GetFileInformation: GetFileInformation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &GUID) -> bool {
        iid == &<IDStorageFile as Interface>::IID
    }
}

impl From<IDStorageFile> for IUnknown {
    fn from(value: IDStorageFile) -> Self {
        unsafe { transmute(value) }
    }
}

impl From<&IDStorageFile> for IUnknown {
    fn from(value: &IDStorageFile) -> Self {
        From::from(Clone::clone(value))
    }
}

impl Clone for IDStorageFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl PartialEq for IDStorageFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for IDStorageFile {}

impl std::fmt::Debug for IDStorageFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("IDStorageFile").field(&self.0).finish()
    }
}

#[repr(transparent)]
pub struct IDStorageCustomDecompressionQueue(IUnknown);

impl IDStorageCustomDecompressionQueue {
    pub unsafe fn GetEvent(&self) -> HANDLE {
        (Vtable::vtable(self).GetEvent)(Vtable::as_raw(self))
    }
    pub unsafe fn GetRequests(
        &self,
        maxRequests: u32,
        requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
        numRequests: *mut u32,
    ) -> Result<()> {
        (Vtable::vtable(self).GetRequests)(Vtable::as_raw(self), maxRequests, requests, numRequests)
            .ok()
    }
    pub unsafe fn SetRequestResults(
        &self,
        numResults: u32,
        requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_RESULT,
    ) -> Result<()> {
        (Vtable::vtable(self).SetRequestResults)(Vtable::as_raw(self), numResults, requests).ok()
    }
}

unsafe impl Vtable for IDStorageCustomDecompressionQueue {
    type Vtable = IDStorageCustomDecompressionQueue_Vtbl;
}

unsafe impl Interface for IDStorageCustomDecompressionQueue {
    const IID: GUID = GUID {
        data1: 0x97179B2F,
        data2: 0x2C21,
        data3: 0x49CA,
        data4: [0x82, 0x91, 0x4E, 0x1B, 0xF4, 0xA1, 0x60, 0xDF],
    };
}

impl RuntimeName for IDStorageCustomDecompressionQueue {}

#[allow(non_camel_case_types)]
pub trait IDStorageCustomDecompressionQueue_Impl: Sized {
    unsafe fn GetEvent(&self) -> HANDLE;
    unsafe fn GetRequests(
        &self,
        maxRequests: u32,
        requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
        numRequests: *mut u32,
    ) -> HRESULT;
    unsafe fn SetRequestResults(
        &self,
        numResults: u32,
        requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_RESULT,
    ) -> HRESULT;
}

#[repr(C)]
#[doc(hidden)]
pub struct IDStorageCustomDecompressionQueue_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub GetEvent: unsafe extern "system" fn(this: *mut c_void) -> HANDLE,
    pub GetRequests: unsafe extern "system" fn(
        this: *mut c_void,
        maxRequests: u32,
        requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
        numRequests: *mut u32,
    ) -> HRESULT,
    pub SetRequestResults: unsafe extern "system" fn(
        this: *mut c_void,
        numResults: u32,
        requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_RESULT,
    ) -> HRESULT,
}

impl IDStorageCustomDecompressionQueue_Vtbl {
    pub const fn new<
        Identity: IUnknownImpl<Impl = Impl>,
        Impl: IDStorageCustomDecompressionQueue_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn GetEvent<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageCustomDecompressionQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
        ) -> HANDLE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEvent()
        }
        unsafe extern "system" fn GetRequests<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageCustomDecompressionQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
            maxRequests: u32,
            requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
            numRequests: *mut u32,
        ) -> HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRequests(maxRequests, requests, numRequests)
        }
        unsafe extern "system" fn SetRequestResults<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageCustomDecompressionQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
            numResults: u32,
            requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_RESULT,
        ) -> HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRequestResults(numResults, requests)
        }
        Self {
            base__: IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEvent: GetEvent::<Identity, Impl, OFFSET>,
            GetRequests: GetRequests::<Identity, Impl, OFFSET>,
            SetRequestResults: SetRequestResults::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &GUID) -> bool {
        iid == &<IDStorageCustomDecompressionQueue as Interface>::IID
    }
}

impl From<IDStorageCustomDecompressionQueue> for IUnknown {
    fn from(value: IDStorageCustomDecompressionQueue) -> Self {
        unsafe { transmute(value) }
    }
}

impl From<&IDStorageCustomDecompressionQueue> for IUnknown {
    fn from(value: &IDStorageCustomDecompressionQueue) -> Self {
        From::from(Clone::clone(value))
    }
}

impl Clone for IDStorageCustomDecompressionQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl PartialEq for IDStorageCustomDecompressionQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for IDStorageCustomDecompressionQueue {}

impl std::fmt::Debug for IDStorageCustomDecompressionQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("IDStorageCustomDecompressionQueue")
            .field(&self.0)
            .finish()
    }
}

#[repr(transparent)]
pub struct IDStorageCustomDecompressionQueue1(IDStorageCustomDecompressionQueue);

impl IDStorageCustomDecompressionQueue1 {
    pub unsafe fn GetRequests1(
        &self,
        flags: DSTORAGE_GET_REQUEST_FLAGS,
        maxRequests: u32,
        requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
        numRequests: *mut u32,
    ) -> HANDLE {
        (Vtable::vtable(self).GetRequests1)(
            Vtable::as_raw(self),
            flags,
            maxRequests,
            requests,
            numRequests,
        )
    }
}

unsafe impl Vtable for IDStorageCustomDecompressionQueue1 {
    type Vtable = IDStorageCustomDecompressionQueue1_Vtbl;
}

unsafe impl Interface for IDStorageCustomDecompressionQueue1 {
    const IID: GUID = GUID {
        data1: 0x0D47C6C9,
        data2: 0xE61A,
        data3: 0x4706,
        data4: [0x93, 0xB4, 0x68, 0xBF, 0xE3, 0xF4, 0xAA, 0x4A],
    };
}

impl RuntimeName for IDStorageCustomDecompressionQueue1 {}

#[allow(non_camel_case_types)]
pub trait IDStorageCustomDecompressionQueue1_Impl:
    Sized + IDStorageCustomDecompressionQueue_Impl
{
    unsafe fn GetRequests1(
        &self,
        flags: DSTORAGE_GET_REQUEST_FLAGS,
        maxRequests: u32,
        requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
        numRequests: *mut u32,
    ) -> HANDLE;
}

#[repr(C)]
#[doc(hidden)]
pub struct IDStorageCustomDecompressionQueue1_Vtbl {
    pub base__: IDStorageCustomDecompressionQueue_Vtbl,
    pub GetRequests1: unsafe extern "system" fn(
        this: *mut c_void,
        flags: DSTORAGE_GET_REQUEST_FLAGS,
        maxRequests: u32,
        requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
        numRequests: *mut u32,
    ) -> HANDLE,
}

impl IDStorageCustomDecompressionQueue1_Vtbl {
    pub const fn new<
        Identity: IUnknownImpl<Impl = Impl>,
        Impl: IDStorageCustomDecompressionQueue1_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn GetRequests1<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageCustomDecompressionQueue1_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
            flags: DSTORAGE_GET_REQUEST_FLAGS,
            maxRequests: u32,
            requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
            numRequests: *mut u32,
        ) -> HANDLE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRequests1(flags, maxRequests, requests, numRequests)
        }
        Self {
            base__: IDStorageCustomDecompressionQueue_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetRequests1: GetRequests1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &GUID) -> bool {
        iid == &<IDStorageCustomDecompressionQueue1 as Interface>::IID
    }
}

impl From<IDStorageCustomDecompressionQueue1> for IUnknown {
    fn from(value: IDStorageCustomDecompressionQueue1) -> Self {
        unsafe { transmute(value) }
    }
}

impl From<&IDStorageCustomDecompressionQueue1> for IUnknown {
    fn from(value: &IDStorageCustomDecompressionQueue1) -> Self {
        From::from(Clone::clone(value))
    }
}

impl Clone for IDStorageCustomDecompressionQueue1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl PartialEq for IDStorageCustomDecompressionQueue1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for IDStorageCustomDecompressionQueue1 {}

impl std::fmt::Debug for IDStorageCustomDecompressionQueue1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("IDStorageCustomDecompressionQueue1")
            .field(&self.0)
            .finish()
    }
}

#[repr(transparent)]
pub struct IDStorageFactory(IUnknown);

impl IDStorageFactory {
    pub unsafe fn CreateQueue<T>(&self, desc: *const DSTORAGE_QUEUE_DESC) -> Result<T>
    where
        T: Interface,
    {
        let mut result = None;
        (Vtable::vtable(self).CreateQueue)(
            Vtable::as_raw(self),
            desc,
            &T::IID,
            &mut result as *mut _ as *mut _,
        )
        .and_some(result)
    }
    pub unsafe fn OpenFile<T>(&self, path: PCWSTR) -> Result<T>
    where
        T: Interface,
    {
        let mut result = None;
        (Vtable::vtable(self).OpenFile)(
            Vtable::as_raw(self),
            path,
            &T::IID,
            &mut result as *mut _ as *mut _,
        )
        .and_some(result)
    }
    pub unsafe fn CreateStatusArray<T>(&self, capacity: u32, name: PCSTR) -> Result<T>
    where
        T: Interface,
    {
        let mut result = None;
        (Vtable::vtable(self).CreateStatusArray)(
            Vtable::as_raw(self),
            capacity,
            name,
            &T::IID,
            &mut result as *mut _ as *mut _,
        )
        .and_some(result)
    }
    pub unsafe fn SetDebugFlags(&self, flags: DSTORAGE_DEBUG) {
        (Vtable::vtable(self).SetDebugFlags)(Vtable::as_raw(self), flags)
    }
    pub unsafe fn SetStagingBufferSize(&self, size: u32) -> Result<()> {
        (Vtable::vtable(self).SetStagingBufferSize)(Vtable::as_raw(self), size).ok()
    }
}

unsafe impl Vtable for IDStorageFactory {
    type Vtable = IDStorageFactory_Vtbl;
}

unsafe impl Interface for IDStorageFactory {
    const IID: GUID = GUID {
        data1: 0x6924EA0C,
        data2: 0xC3CD,
        data3: 0x4826,
        data4: [0xB1, 0x0A, 0xF6, 0x4F, 0x4E, 0xD9, 0x27, 0xC1],
    };
}

impl RuntimeName for IDStorageFactory {}

#[allow(non_camel_case_types)]
pub trait IDStorageFactory_Impl: Sized {
    unsafe fn CreateQueue(
        &self,
        desc: *const DSTORAGE_QUEUE_DESC,
        riid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    unsafe fn OpenFile(&self, path: PCWSTR, riid: *const GUID, ppv: *mut *mut c_void) -> HRESULT;
    unsafe fn CreateStatusArray(
        &self,
        capacity: u32,
        name: PCSTR,
        riid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    unsafe fn SetDebugFlags(&self, flags: DSTORAGE_DEBUG);
    unsafe fn SetStagingBufferSize(&self, size: u32) -> HRESULT;
}

#[repr(C)]
#[doc(hidden)]
pub struct IDStorageFactory_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub CreateQueue: unsafe extern "system" fn(
        this: *mut c_void,
        desc: *const DSTORAGE_QUEUE_DESC,
        riid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    pub OpenFile: unsafe extern "system" fn(
        this: *mut c_void,
        path: PCWSTR,
        riid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    pub CreateStatusArray: unsafe extern "system" fn(
        this: *mut c_void,
        capacity: u32,
        name: PCSTR,
        riid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    pub SetDebugFlags: unsafe extern "system" fn(this: *mut c_void, flags: DSTORAGE_DEBUG),
    pub SetStagingBufferSize: unsafe extern "system" fn(this: *mut c_void, size: u32) -> HRESULT,
}

impl IDStorageFactory_Vtbl {
    pub const fn new<
        Identity: IUnknownImpl<Impl = Impl>,
        Impl: IDStorageFactory_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn CreateQueue<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
            desc: *const DSTORAGE_QUEUE_DESC,
            riid: *const GUID,
            ppv: *mut *mut c_void,
        ) -> HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateQueue(desc, riid, ppv)
        }
        unsafe extern "system" fn OpenFile<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
            path: PCWSTR,
            riid: *const GUID,
            ppv: *mut *mut c_void,
        ) -> HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenFile(path, riid, ppv)
        }
        unsafe extern "system" fn CreateStatusArray<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
            capacity: u32,
            name: PCSTR,
            riid: *const GUID,
            ppv: *mut *mut c_void,
        ) -> HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateStatusArray(capacity, name, riid, ppv)
        }
        unsafe extern "system" fn SetDebugFlags<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
            flags: DSTORAGE_DEBUG,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDebugFlags(flags)
        }
        unsafe extern "system" fn SetStagingBufferSize<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
            size: u32,
        ) -> HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStagingBufferSize(size)
        }
        Self {
            base__: IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateQueue: CreateQueue::<Identity, Impl, OFFSET>,
            OpenFile: OpenFile::<Identity, Impl, OFFSET>,
            CreateStatusArray: CreateStatusArray::<Identity, Impl, OFFSET>,
            SetDebugFlags: SetDebugFlags::<Identity, Impl, OFFSET>,
            SetStagingBufferSize: SetStagingBufferSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &GUID) -> bool {
        iid == &<IDStorageFactory as Interface>::IID
    }
}

impl From<IDStorageFactory> for IUnknown {
    fn from(value: IDStorageFactory) -> Self {
        unsafe { transmute(value) }
    }
}

impl From<&IDStorageFactory> for IUnknown {
    fn from(value: &IDStorageFactory) -> Self {
        From::from(Clone::clone(value))
    }
}

impl Clone for IDStorageFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl PartialEq for IDStorageFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for IDStorageFactory {}

impl std::fmt::Debug for IDStorageFactory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("IDStorageFactory").field(&self.0).finish()
    }
}

#[repr(transparent)]
pub struct IDStorageStatusArray(IUnknown);

impl IDStorageStatusArray {
    pub unsafe fn IsComplete(&self, index: u32) -> bool {
        (Vtable::vtable(self).IsComplete)(Vtable::as_raw(self), index)
    }
    pub unsafe fn GetHResult(&self, index: u32) -> Result<()> {
        (Vtable::vtable(self).GetHResult)(Vtable::as_raw(self), index).ok()
    }
}

unsafe impl Vtable for IDStorageStatusArray {
    type Vtable = IDStorageStatusArray_Vtbl;
}

unsafe impl Interface for IDStorageStatusArray {
    const IID: GUID = GUID {
        data1: 0x82397587,
        data2: 0x7CD5,
        data3: 0x453B,
        data4: [0xA0, 0x2E, 0x31, 0x37, 0x9B, 0xD6, 0x46, 0x56],
    };
}

impl RuntimeName for IDStorageStatusArray {}

#[allow(non_camel_case_types)]
pub trait IDStorageStatusArray_Impl: Sized {
    unsafe fn IsComplete(&self, index: u32) -> bool;
    unsafe fn GetHResult(&self, index: u32) -> HRESULT;
}

#[repr(C)]
#[doc(hidden)]
pub struct IDStorageStatusArray_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub IsComplete: unsafe extern "system" fn(this: *mut c_void, index: u32) -> bool,
    pub GetHResult: unsafe extern "system" fn(this: *mut c_void, index: u32) -> HRESULT,
}

impl IDStorageStatusArray_Vtbl {
    pub const fn new<
        Identity: IUnknownImpl<Impl = Impl>,
        Impl: IDStorageStatusArray_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn IsComplete<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageStatusArray_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
            index: u32,
        ) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsComplete(index)
        }
        unsafe extern "system" fn GetHResult<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageStatusArray_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
            index: u32,
        ) -> HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHResult(index)
        }
        Self {
            base__: IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsComplete: IsComplete::<Identity, Impl, OFFSET>,
            GetHResult: GetHResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &GUID) -> bool {
        iid == &<IDStorageStatusArray as Interface>::IID
    }
}

impl From<IDStorageStatusArray> for IUnknown {
    fn from(value: IDStorageStatusArray) -> Self {
        unsafe { transmute(value) }
    }
}

impl From<&IDStorageStatusArray> for IUnknown {
    fn from(value: &IDStorageStatusArray) -> Self {
        From::from(Clone::clone(value))
    }
}

impl Clone for IDStorageStatusArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl PartialEq for IDStorageStatusArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for IDStorageStatusArray {}

impl std::fmt::Debug for IDStorageStatusArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("IDStorageStatusArray")
            .field(&self.0)
            .finish()
    }
}

#[repr(transparent)]
pub struct IDStorageQueue(IUnknown);

impl IDStorageQueue {
    pub unsafe fn EnqueueRequest(&self, request: *const DSTORAGE_REQUEST) {
        (Vtable::vtable(self).EnqueueRequest)(Vtable::as_raw(self), request)
    }
    pub unsafe fn EnqueueStatus(&self, statusArray: *mut IDStorageStatusArray, index: u32) {
        (Vtable::vtable(self).EnqueueStatus)(Vtable::as_raw(self), statusArray, index)
    }
    pub unsafe fn EnqueueSignal(&self, fence: *mut ID3D12Fence, value: u64) {
        (Vtable::vtable(self).EnqueueSignal)(Vtable::as_raw(self), fence, value)
    }
    pub unsafe fn Submit(&self) {
        (Vtable::vtable(self).Submit)(Vtable::as_raw(self))
    }
    pub unsafe fn CancelRequestsWithTag(&self, mask: u64, value: u64) {
        (Vtable::vtable(self).CancelRequestsWithTag)(Vtable::as_raw(self), mask, value)
    }
    pub unsafe fn Close(&self) {
        (Vtable::vtable(self).Close)(Vtable::as_raw(self))
    }
    pub unsafe fn GetErrorEvent(&self) -> HANDLE {
        (Vtable::vtable(self).GetErrorEvent)(Vtable::as_raw(self))
    }
    pub unsafe fn RetrieveErrorRecord(&self) -> DSTORAGE_ERROR_RECORD {
        let mut record = MaybeUninit::zeroed();
        (Vtable::vtable(self).RetrieveErrorRecord)(Vtable::as_raw(self), record.as_mut_ptr());
        record.assume_init()
    }
    pub unsafe fn Query(&self, info: *mut DSTORAGE_QUEUE_INFO) {
        (Vtable::vtable(self).Query)(Vtable::as_raw(self), info)
    }
}

unsafe impl Vtable for IDStorageQueue {
    type Vtable = IDStorageQueue_Vtbl;
}

unsafe impl Interface for IDStorageQueue {
    const IID: GUID = GUID {
        data1: 0xCFDBD83F,
        data2: 0x9E06,
        data3: 0x4FDA,
        data4: [0x8E, 0xA5, 0x69, 0x04, 0x21, 0x37, 0xF4, 0x9B],
    };
}

impl RuntimeName for IDStorageQueue {}

#[allow(non_camel_case_types)]
pub trait IDStorageQueue_Impl: Sized {
    unsafe fn EnqueueRequest(&self, request: *const DSTORAGE_REQUEST);
    unsafe fn EnqueueStatus(&self, statusArray: *mut IDStorageStatusArray, index: u32);
    unsafe fn EnqueueSignal(&self, fence: *mut ID3D12Fence, value: u64);
    unsafe fn Submit(&self);
    unsafe fn CancelRequestsWithTag(&self, mask: u64, value: u64);
    unsafe fn Close(&self);
    unsafe fn GetErrorEvent(&self) -> HANDLE;
    unsafe fn RetrieveErrorRecord(&self, record: *mut DSTORAGE_ERROR_RECORD);
    unsafe fn Query(&self, info: *mut DSTORAGE_QUEUE_INFO);
}

#[repr(C)]
#[doc(hidden)]
pub struct IDStorageQueue_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub EnqueueRequest:
        unsafe extern "system" fn(this: *mut c_void, request: *const DSTORAGE_REQUEST),
    pub EnqueueStatus: unsafe extern "system" fn(
        this: *mut c_void,
        statusArray: *mut IDStorageStatusArray,
        index: u32,
    ),
    pub EnqueueSignal:
        unsafe extern "system" fn(this: *mut c_void, fence: *mut ID3D12Fence, value: u64),
    pub Submit: unsafe extern "system" fn(this: *mut c_void),
    pub CancelRequestsWithTag: unsafe extern "system" fn(this: *mut c_void, mask: u64, value: u64),
    pub Close: unsafe extern "system" fn(this: *mut c_void),
    pub GetErrorEvent: unsafe extern "system" fn(this: *mut c_void) -> HANDLE,
    pub RetrieveErrorRecord:
        unsafe extern "system" fn(this: *mut c_void, record: *mut DSTORAGE_ERROR_RECORD),
    pub Query: unsafe extern "system" fn(this: *mut c_void, info: *mut DSTORAGE_QUEUE_INFO),
}

impl IDStorageQueue_Vtbl {
    pub const fn new<
        Identity: IUnknownImpl<Impl = Impl>,
        Impl: IDStorageQueue_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn EnqueueRequest<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
            request: *const DSTORAGE_REQUEST,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnqueueRequest(request)
        }
        unsafe extern "system" fn EnqueueStatus<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
            statusArray: *mut IDStorageStatusArray,
            index: u32,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnqueueStatus(statusArray, index)
        }
        unsafe extern "system" fn EnqueueSignal<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
            fence: *mut ID3D12Fence,
            value: u64,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnqueueSignal(fence, value)
        }
        unsafe extern "system" fn Submit<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Submit()
        }
        unsafe extern "system" fn CancelRequestsWithTag<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
            mask: u64,
            value: u64,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelRequestsWithTag(mask, value)
        }
        unsafe extern "system" fn Close<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close()
        }
        unsafe extern "system" fn GetErrorEvent<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
        ) -> HANDLE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetErrorEvent()
        }
        unsafe extern "system" fn RetrieveErrorRecord<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
            record: *mut DSTORAGE_ERROR_RECORD,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RetrieveErrorRecord(record)
        }
        unsafe extern "system" fn Query<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
            info: *mut DSTORAGE_QUEUE_INFO,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Query(info)
        }
        Self {
            base__: IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnqueueRequest: EnqueueRequest::<Identity, Impl, OFFSET>,
            EnqueueStatus: EnqueueStatus::<Identity, Impl, OFFSET>,
            EnqueueSignal: EnqueueSignal::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
            CancelRequestsWithTag: CancelRequestsWithTag::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            GetErrorEvent: GetErrorEvent::<Identity, Impl, OFFSET>,
            RetrieveErrorRecord: RetrieveErrorRecord::<Identity, Impl, OFFSET>,
            Query: Query::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &GUID) -> bool {
        iid == &<IDStorageQueue as Interface>::IID
    }
}

impl From<IDStorageQueue> for IUnknown {
    fn from(value: IDStorageQueue) -> Self {
        unsafe { transmute(value) }
    }
}

impl From<&IDStorageQueue> for IUnknown {
    fn from(value: &IDStorageQueue) -> Self {
        From::from(Clone::clone(value))
    }
}

impl Clone for IDStorageQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl PartialEq for IDStorageQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for IDStorageQueue {}

impl std::fmt::Debug for IDStorageQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("IDStorageQueue").field(&self.0).finish()
    }
}

#[repr(transparent)]
pub struct IDStorageQueue1(IDStorageQueue);

impl IDStorageQueue1 {
    pub unsafe fn EnqueueSetEvent(&self, handle: HANDLE) {
        (Vtable::vtable(self).EnqueueSetEvent)(Vtable::as_raw(self), handle)
    }
}

unsafe impl Vtable for IDStorageQueue1 {
    type Vtable = IDStorageQueue1_Vtbl;
}

unsafe impl Interface for IDStorageQueue1 {
    const IID: GUID = GUID {
        data1: 0xDD2F482C,
        data2: 0x5EFF,
        data3: 0x41E8,
        data4: [0x9C, 0x9E, 0xD2, 0x37, 0x4B, 0x27, 0x81, 0x28],
    };
}

impl RuntimeName for IDStorageQueue1 {}

#[allow(non_camel_case_types)]
pub trait IDStorageQueue1_Impl: Sized + IDStorageQueue_Impl {
    unsafe fn EnqueueSetEvent(&self, handle: HANDLE);
}

#[repr(C)]
#[doc(hidden)]
pub struct IDStorageQueue1_Vtbl {
    pub base__: IDStorageQueue_Vtbl,
    pub EnqueueSetEvent: unsafe extern "system" fn(this: *mut c_void, handle: HANDLE),
}

impl IDStorageQueue1_Vtbl {
    pub const fn new<
        Identity: IUnknownImpl<Impl = Impl>,
        Impl: IDStorageQueue1_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn EnqueueSetEvent<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageQueue1_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
            handle: HANDLE,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnqueueSetEvent(handle)
        }
        Self {
            base__: IDStorageQueue_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnqueueSetEvent: EnqueueSetEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &GUID) -> bool {
        iid == &<IDStorageQueue1 as Interface>::IID
    }
}

impl From<IDStorageQueue1> for IUnknown {
    fn from(value: IDStorageQueue1) -> Self {
        unsafe { transmute(value) }
    }
}

impl From<&IDStorageQueue1> for IUnknown {
    fn from(value: &IDStorageQueue1) -> Self {
        From::from(Clone::clone(value))
    }
}

impl Clone for IDStorageQueue1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl PartialEq for IDStorageQueue1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for IDStorageQueue1 {}

impl std::fmt::Debug for IDStorageQueue1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("IDStorageQueue1").field(&self.0).finish()
    }
}

#[repr(transparent)]
pub struct IDStorageCompressionCodec(IUnknown);

impl IDStorageCompressionCodec {
    pub unsafe fn CompressBuffer(
        &self,
        uncompressedData: *const c_void,
        uncompressedDataSize: c_size_t,
        compressionSetting: DSTORAGE_COMPRESSION,
        compressedBuffer: *mut c_void,
        compressedBufferSize: c_size_t,
        compressedDataSize: *mut c_size_t,
    ) -> Result<()> {
        (Vtable::vtable(self).CompressBuffer)(
            Vtable::as_raw(self),
            uncompressedData,
            uncompressedDataSize,
            compressionSetting,
            compressedBuffer,
            compressedBufferSize,
            compressedDataSize,
        )
        .ok()
    }
    pub unsafe fn DecompressBuffer(
        &self,
        compressedData: *const c_void,
        compressedDataSize: c_size_t,
        uncompressedBuffer: *mut c_void,
        uncompressedBufferSize: c_size_t,
        uncompressedDataSize: c_size_t,
    ) -> Result<()> {
        (Vtable::vtable(self).DecompressBuffer)(
            Vtable::as_raw(self),
            compressedData,
            compressedDataSize,
            uncompressedBuffer,
            uncompressedBufferSize,
            uncompressedDataSize,
        )
        .ok()
    }
    pub unsafe fn CompressBufferBound(&self, uncompressedDataSize: c_size_t) -> c_size_t {
        (Vtable::vtable(self).CompressBufferBound)(Vtable::as_raw(self), uncompressedDataSize)
    }
}

unsafe impl Vtable for IDStorageCompressionCodec {
    type Vtable = IDStorageCompressionCodec_Vtbl;
}

unsafe impl Interface for IDStorageCompressionCodec {
    const IID: GUID = GUID {
        data1: 0x84EF5121,
        data2: 0x9B43,
        data3: 0x4D03,
        data4: [0xB5, 0xC1, 0xCC, 0x34, 0x60, 0x6B, 0x26, 0x2D],
    };
}

impl RuntimeName for IDStorageCompressionCodec {}

#[allow(non_camel_case_types)]
pub trait IDStorageCompressionCodec_Impl: Sized {
    unsafe fn CompressBuffer(
        &self,
        uncompressedData: *const c_void,
        uncompressedDataSize: c_size_t,
        compressionSetting: DSTORAGE_COMPRESSION,
        compressedBuffer: *mut c_void,
        compressedBufferSize: c_size_t,
        compressedDataSize: *mut c_size_t,
    ) -> HRESULT;
    unsafe fn DecompressBuffer(
        &self,
        compressedData: *const c_void,
        compressedDataSize: c_size_t,
        uncompressedBuffer: *mut c_void,
        uncompressedBufferSize: c_size_t,
        uncompressedDataSize: c_size_t,
    ) -> HRESULT;
    unsafe fn CompressBufferBound(&self, uncompressedDataSize: c_size_t) -> c_size_t;
}

#[repr(C)]
#[doc(hidden)]
pub struct IDStorageCompressionCodec_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub CompressBuffer: unsafe extern "system" fn(
        this: *mut c_void,
        uncompressedData: *const c_void,
        uncompressedDataSize: c_size_t,
        compressionSetting: DSTORAGE_COMPRESSION,
        compressedBuffer: *mut c_void,
        compressedBufferSize: c_size_t,
        compressedDataSize: *mut c_size_t,
    ) -> HRESULT,
    pub DecompressBuffer: unsafe extern "system" fn(
        this: *mut c_void,
        compressedData: *const c_void,
        compressedDataSize: c_size_t,
        uncompressedBuffer: *mut c_void,
        uncompressedBufferSize: c_size_t,
        uncompressedDataSize: c_size_t,
    ) -> HRESULT,
    pub CompressBufferBound:
        unsafe extern "system" fn(this: *mut c_void, uncompressedDataSize: c_size_t) -> c_size_t,
}

impl IDStorageCompressionCodec_Vtbl {
    pub const fn new<
        Identity: IUnknownImpl<Impl = Impl>,
        Impl: IDStorageCompressionCodec_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn CompressBuffer<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageCompressionCodec_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
            uncompressedData: *const c_void,
            uncompressedDataSize: c_size_t,
            compressionSetting: DSTORAGE_COMPRESSION,
            compressedBuffer: *mut c_void,
            compressedBufferSize: c_size_t,
            compressedDataSize: *mut c_size_t,
        ) -> HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompressBuffer(
                uncompressedData,
                uncompressedDataSize,
                compressionSetting,
                compressedBuffer,
                compressedBufferSize,
                compressedDataSize,
            )
        }
        unsafe extern "system" fn DecompressBuffer<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageCompressionCodec_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
            compressedData: *const c_void,
            compressedDataSize: c_size_t,
            uncompressedBuffer: *mut c_void,
            uncompressedBufferSize: c_size_t,
            uncompressedDataSize: c_size_t,
        ) -> HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DecompressBuffer(
                compressedData,
                compressedDataSize,
                uncompressedBuffer,
                uncompressedBufferSize,
                uncompressedDataSize,
            )
        }
        unsafe extern "system" fn CompressBufferBound<
            Identity: IUnknownImpl<Impl = Impl>,
            Impl: IDStorageCompressionCodec_Impl,
            const OFFSET: isize,
        >(
            this: *mut c_void,
            uncompressedDataSize: c_size_t,
        ) -> c_size_t {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompressBufferBound(uncompressedDataSize)
        }
        Self {
            base__: IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CompressBuffer: CompressBuffer::<Identity, Impl, OFFSET>,
            DecompressBuffer: DecompressBuffer::<Identity, Impl, OFFSET>,
            CompressBufferBound: CompressBufferBound::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &GUID) -> bool {
        iid == &<IDStorageCompressionCodec as Interface>::IID
    }
}

impl From<IDStorageCompressionCodec> for IUnknown {
    fn from(value: IDStorageCompressionCodec) -> Self {
        unsafe { transmute(value) }
    }
}

impl From<&IDStorageCompressionCodec> for IUnknown {
    fn from(value: &IDStorageCompressionCodec) -> Self {
        From::from(Clone::clone(value))
    }
}

impl Clone for IDStorageCompressionCodec {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl PartialEq for IDStorageCompressionCodec {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for IDStorageCompressionCodec {}

impl std::fmt::Debug for IDStorageCompressionCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("IDStorageCompressionCodec")
            .field(&self.0)
            .finish()
    }
}

// These definitions are used to automatically implement the COM objects when windows_rs changes
// their implementation details. We do this for three reasons:
//     1. We don't want to include syn / quote as a dependency.
//     2. The official proc macros is messing up IDE linter.
//     3. We want to adapt the improved interface that windows_rs exposes.

/*
#[windows::core::interface("5de95e7b-955a-4868-a73c-243b29f4b8da")]
pub unsafe trait IDStorageFile: IUnknown {
    pub unsafe fn Close(&self);
    pub unsafe fn GetFileInformation(&self, info: *mut BY_HANDLE_FILE_INFORMATION) -> HRESULT;
}

#[windows::core::interface("97179b2f-2c21-49ca-8291-4e1bf4a160df")]
pub unsafe trait IDStorageCustomDecompressionQueue: IUnknown {
    pub unsafe fn GetEvent(&self) -> HANDLE;
    pub unsafe fn GetRequests(
        &self,
        maxRequests: u32,
        requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
        numRequests: *mut u32,
    ) -> HRESULT;
    pub unsafe fn SetRequestResults(
        &self,
        numResults: u32,
        requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_RESULT,
    ) -> HRESULT;
}

#[windows::core::interface("0D47C6C9-E61A-4706-93B4-68BFE3F4AA4A")]
pub unsafe trait IDStorageCustomDecompressionQueue1:
    IDStorageCustomDecompressionQueue
{
    pub unsafe fn GetRequests1(
        &self,
        flags: DSTORAGE_GET_REQUEST_FLAGS,
        maxRequests: u32,
        requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
        numRequests: *mut u32,
    ) -> HANDLE;
}

#[windows::core::interface("6924ea0c-c3cd-4826-b10a-f64f4ed927c1")]
pub unsafe trait IDStorageFactory: IUnknown {
    pub unsafe fn CreateQueue(
        &self,
        desc: *const DSTORAGE_QUEUE_DESC,
        riid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub unsafe fn OpenFile(
        &self,
        path: PCWSTR,
        riid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub unsafe fn CreateStatusArray(
        &self,
        capacity: u32,
        name: PCSTR,
        riid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub unsafe fn SetDebugFlags(&self, flags: DSTORAGE_GET_REQUEST_FLAGS);
    pub unsafe fn SetStagingBufferSize(&self, size: u32) -> HRESULT;
}

#[windows::core::interface("82397587-7cd5-453b-a02e-31379bd64656")]
pub unsafe trait IDStorageStatusArray: IUnknown {
    pub unsafe fn IsComplete(&self, index: u32) -> bool;
    pub unsafe fn GetHResult(&self, index: u32) -> HRESULT;
}

#[windows::core::interface("cfdbd83f-9e06-4fda-8ea5-69042137f49b")]
pub unsafe trait IDStorageQueue: IUnknown {
    pub unsafe fn EnqueueRequest(&self, request: *const DSTORAGE_REQUEST);
    pub unsafe fn EnqueueStatus(&self, statusArray: *mut IDStorageStatusArray, index: u32);
    pub unsafe fn EnqueueSignal(&self, fence: *mut ID3D12Fence, value: u64);
    pub unsafe fn Submit(&self);
    pub unsafe fn CancelRequestsWithTag(&self, mask: u64, value: u64);
    pub unsafe fn Close(&self);
    pub unsafe fn GetErrorEvent(&self) -> HANDLE;
    pub unsafe fn RetrieveErrorRecord(&self, record: *mut DSTORAGE_ERROR_RECORD);
    pub unsafe fn Query(&self, info: *mut DSTORAGE_QUEUE_INFO);
}

#[windows::core::interface("dd2f482c-5eff-41e8-9c9e-d2374b278128")]
pub unsafe trait IDStorageQueue1: IDStorageQueue {
    pub unsafe fn EnqueueSetEvent(&self, handle: HANDLE);
}

#[windows::core::interface("84ef5121-9b43-4d03-b5c1-cc34606b262d")]
pub unsafe trait IDStorageCompressionCodec: IUnknown {
    pub unsafe fn CompressBuffer(
        &self,
        uncompressedData: *const c_void,
        uncompressedDataSize: c_size_t,
        compressionSetting: DSTORAGE_COMPRESSION,
        compressedBuffer: *mut c_void,
        compressedBufferSize: c_size_t,
        compressedDataSize: *mut c_size_t,
    ) -> HRESULT;
    pub unsafe fn DecompressBuffer(
        &self,
        compressedData: *const c_void,
        compressedDataSize: c_size_t,
        uncompressedBuffer: *mut c_void,
        uncompressedBufferSize: c_size_t,
        uncompressedDataSize: c_size_t,
    ) -> HRESULT;
    pub unsafe fn CompressBufferBound(&self, uncompressedDataSize: c_size_t) -> c_size_t;
}
*/
