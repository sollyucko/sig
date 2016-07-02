// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/sig
//
// This file may not be copied, modified, or distributed
// except according to those terms.

//! # Sig
//!
//! [![Crate][crate-badge]][crate] [![license-badge][]][license] [![travis-badge][]][travis] [![circle-badge][]][circle]
//!
//! [crate-badge]: https://img.shields.io/badge/crates.io-v0.1.1-orange.svg?style=flat-square
//! [crate]: https://crates.io/crates/sig
//! [license-badge]: https://img.shields.io/crates/l/cublas.svg?style=flat-square
//! [license]: https://github.com/adjivas/sig/blob/master/README.md#license
//! [travis-badge]: https://travis-ci.org/adjivas/sig.svg?style=flat-square
//! [travis]: https://travis-ci.org/adjivas/sig
//! [circle-badge]: https://circleci.com/gh/adjivas/sig/tree/master.svg?style=svg
//! [circle]: https://circleci.com/gh/adjivas/sig/tree/master

extern crate libc;

#[macro_use]
mod macros;
pub mod ffi;

#[inline]
pub unsafe fn set_signal_handler(signal: ffi::c_int,
                                 handler: unsafe extern "C" fn(ffi::c_int)) {
    use libc::{sigaction, sighandler_t, sigfillset};
    use std::{mem, ptr};

    let mut sigset = mem::uninitialized();

    // Block all signals during the handler. This is the expected behavior, but
    // it's not guaranteed by `signal()`.
    let rv = sigfillset(&mut sigset);
    if rv == -1 {
        return;
    }

    // Done because sigaction has private members.
    // This is safe because sa_restorer and sa_handlers are pointers that
    // might be null (that is, zero).
    let mut action: sigaction = mem::zeroed();

    // action.sa_flags = 0;
    action.sa_mask = sigset;
    action.sa_sigaction = handler as sighandler_t;

    sigaction(signal, &action, ptr::null_mut());
}