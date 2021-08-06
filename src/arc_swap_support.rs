use arc_swap::RefCnt;

use core::ffi::c_void;
use core::ptr::NonNull;
use crate::{Arc, ThinArc};

unsafe impl<H, T> RefCnt for ThinArc<H, T> {
    type Base = c_void;

    #[inline]
    fn into_ptr(me: Self) -> *mut Self::Base {
        ThinArc::into_raw(me).as_ptr()
    }

    #[inline]
    fn as_ptr(me: &Self) -> *mut Self::Base {
        ThinArc::as_ptr(me) as *mut _
    }

    #[inline]
    unsafe fn from_ptr(ptr: *const Self::Base) -> Self {
        ThinArc::from_raw(NonNull::new_unchecked(ptr as *mut _))
    }
}

unsafe impl<T> RefCnt for Arc<T> {
    type Base = T;

    #[inline]
    fn into_ptr(me: Self) -> *mut Self::Base {
        Arc::into_raw(me) as *mut _
    }

    #[inline]
    fn as_ptr(me: &Self) -> *mut Self::Base {
        Arc::as_ptr(me) as *mut _
    }

    #[inline]
    unsafe fn from_ptr(ptr: *const Self::Base) -> Self {
        Arc::from_raw(ptr)
    }
}
