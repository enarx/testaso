// SPDX-License-Identifier: MIT

//! Macro to test alignment, size and offsets of structs
//!
//! This is mostly useful for creating FFI structures.
//!
//! The crucial field offset calculation was extracted from the `memoffset` crate.
//! Kudos to Gilad Naaman and Ralf Jung and all the other contributors.
//!
//! # Examples
//! ```
//! #[repr(C)]
//! struct Simple {
//!     a: u32,
//!     b: [u8; 2],
//!     c: i64,
//! }
//!
//! #[repr(C, packed)]
//! struct SimplePacked {
//!     a: u32,
//!     b: [u8; 2],
//!     c: i64,
//! }
//! #[cfg(test)]
//! mod test {
//!     use testaso::testaso;
//!
//!     use super::Simple;
//!     use super::SimplePacked;
//!
//!     testaso! {
//!         struct Simple: 8, 16 => {
//!             a: 0,
//!             b: 4,
//!             c: 8
//!         }
//!
//!         struct SimplePacked: 1, 14 => {
//!             a: 0,
//!             b: 4,
//!             c: 6
//!         }
//!     }
//! }
//! ```

#![cfg_attr(not(test), no_std)]

/// Macro to test alignment, size and offsets of structs
///
/// # Examples
/// ```
/// #[repr(C)]
/// struct Simple {
///     a: u32,
///     b: [u8; 2],
///     c: i64,
/// }
///
/// #[repr(C, packed)]
/// struct SimplePacked {
///     a: u32,
///     b: [u8; 2],
///     c: i64,
/// }
///
/// #[cfg(test)]
/// mod test {
///     use testaso::testaso;
///
///     use super::Simple;
///     use super::SimplePacked;
///
///     testaso! {
///         struct Simple: 8, 16 => {
///             a: 0,
///             b: 4,
///             c: 8
///         }
///
///         struct SimplePacked: 1, 14 => {
///             a: 0,
///             b: 4,
///             c: 6
///         }
///     }
/// }
#[macro_export]
macro_rules! testaso {
    (@off $name:path=>$field:ident) => { {
        // The following was extracted from the `memoffset` crate:

        // No UB here, and the pointer does not dangle, either.
        // But we have to make sure that `uninit` lives long enough,
        // so it has to be in the same scope as `$name`. That's why
        // this declares a variable (several, actually).
        let uninit = core::mem::MaybeUninit::<$name>::uninit();
        let base_ptr: *const $name = uninit.as_ptr();

        // Make sure the field actually exists. This line ensures that a
        // compile-time error is generated if $field is accessed through a
        // Deref impl.
        #[allow(clippy::unneeded_field_pattern)]
        let $name { $field: _, .. };

        // Get the field address.
        // SAFETY:
        // Crucially, we know that this will not trigger a deref coercion because
        // of the field check we did above.
        // Also, the pointer does not dangle.
        let field_ptr = unsafe { core::ptr::addr_of!((*base_ptr).$field) };

        (field_ptr as usize) - (base_ptr as usize)
    }};

    ($(struct $name:path: $align:expr, $size:expr => { $($field:ident: $offset:expr),* })+) => {
        #[cfg(test)]
        #[test]
        fn align() {
            use core::mem::align_of;

            $(
                assert_eq!(
                    align_of::<$name>(),
                    $align,
                    "align: {}",
                    stringify!($name)
                );
            )+
        }

        #[cfg(test)]
        #[test]
        fn size() {
            use core::mem::size_of;

            $(
                assert_eq!(
                    size_of::<$name>(),
                    $size,
                    "size: {}",
                    stringify!($name)
                );
            )+
        }

        #[cfg(test)]
        #[test]
        fn offsets() {
            $(
                $(
                    assert_eq!(
                        testaso!(@off $name=>$field),
                        $offset,
                        "offset: {}::{}",
                        stringify!($name),
                        stringify!($field)
                    );
                )*
        )+
        }
    };
}

#[cfg(test)]
mod test {
    #[repr(C)]
    struct Simple {
        a: u32,
        b: [u8; 2],
        c: i64,
    }

    #[repr(C, packed)]
    struct SimplePacked {
        a: u32,
        b: [u8; 2],
        c: i64,
    }

    #[repr(C, packed(4))]
    pub struct StructPacked {
        a: u64,
        b: u64,
    }

    #[repr(C)]
    pub struct StructUnPacked {
        a: u64,
        b: u64,
    }

    mod sub {
        #[repr(C)]
        pub struct Simple {
            pub x: u32,
        }
    }

    testaso! {
        struct Simple: 8, 16 => {
            a: 0,
            b: 4,
            c: 8
        }

        struct SimplePacked: 1, 14 => {
            a: 0,
            b: 4,
            c: 6
        }

        struct sub::Simple: 4, 4 => {
            x: 0
        }

        struct StructPacked: 4, 16 => {
            a: 0,
            b: 8
        }

        struct StructUnPacked: 8, 16 => {
            a: 0,
            b: 8
        }
    }
}
