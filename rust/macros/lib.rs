// SPDX-License-Identifier: GPL-2.0

//! Crate for all kernel procedural macros.

mod helpers;
mod module;
mod vtable;

use proc_macro::TokenStream;

/// Declares a kernel module.
///
/// The `type` argument should be a type which implements the [`Module`]
/// trait. Also accepts various forms of kernel metadata.
///
/// C header: [`include/linux/moduleparam.h`](../../../include/linux/moduleparam.h)
///
/// [`Module`]: ../kernel/trait.Module.html
///
/// # Examples
///
/// ```ignore
/// use kernel::prelude::*;
///
/// module!{
///     type: MyModule,
///     name: b"my_kernel_module",
///     author: b"Rust for Linux Contributors",
///     description: b"My very own kernel module!",
///     license: b"GPL",
///     params: {
///        my_i32: i32 {
///            default: 42,
///            permissions: 0o000,
///            description: b"Example of i32",
///        },
///        writeable_i32: i32 {
///            default: 42,
///            permissions: 0o644,
///            description: b"Example of i32",
///        },
///    },
/// }
///
/// struct MyModule;
///
/// impl kernel::Module for MyModule {
///     fn init() -> Result<Self> {
///         // If the parameter is writeable, then the kparam lock must be
///         // taken to read the parameter:
///         {
///             let lock = THIS_MODULE.kernel_param_lock();
///             pr_info!("i32 param is:  {}\n", writeable_i32.read(&lock));
///         }
///         // If the parameter is read only, it can be read without locking
///         // the kernel parameters:
///         pr_info!("i32 param is:  {}\n", my_i32.read());
///         Ok(Self)
///     }
/// }
/// ```
///
/// # Supported argument types
///   - `type`: type which implements the [`Module`] trait (required).
///   - `name`: byte array of the name of the kernel module (required).
///   - `author`: byte array of the author of the kernel module.
///   - `description`: byte array of the description of the kernel module.
///   - `license`: byte array of the license of the kernel module (required).
///   - `alias`: byte array of alias name of the kernel module.
///   - `alias_rtnl_link`: byte array of the `rtnl_link_alias` of the kernel module (mutually exclusive with `alias`).
///   - `params`: parameters for the kernel module, as described below.
///
/// # Supported parameter types
///
///   - `bool`: Corresponds to C `bool` param type.
///   - `i8`: No equivalent C param type.
///   - `u8`: Corresponds to C `char` param type.
///   - `i16`: Corresponds to C `short` param type.
///   - `u16`: Corresponds to C `ushort` param type.
///   - `i32`: Corresponds to C `int` param type.
///   - `u32`: Corresponds to C `uint` param type.
///   - `i64`: No equivalent C param type.
///   - `u64`: Corresponds to C `ullong` param type.
///   - `isize`: No equivalent C param type.
///   - `usize`: No equivalent C param type.
///   - `str`: Corresponds to C `charp` param type. Reading returns a byte slice.
///   - `ArrayParam<T,N>`: Corresponds to C parameters created using `module_param_array`. An array
///     of `T`'s of length at **most** `N`.
///
/// `invbool` is unsupported: it was only ever used in a few modules.
/// Consider using a `bool` and inverting the logic instead.
#[proc_macro]
pub fn module(ts: TokenStream) -> TokenStream {
    module::module(ts)
}

/// Declares or implements a vtable trait.
///
/// Linux's use of pure vtables is very close to Rust traits, but they differ
/// in how unimplemented functions are represented. In Rust, traits can provide
/// default implementation for all non-required methods (and the default
/// implementation could just return `Error::EINVAL`); Linux typically use C
/// `NULL` pointers to represent these functions.
///
/// This attribute is intended to close the gap. Traits can be declared and
/// implemented with the `#[vtable]` attribute, and a `HAS_*` associated constant
/// will be generated for each method in the trait, indicating if the implementor
/// has overriden a method.
///
/// This attribute is not needed if all methods are required.
///
/// # Examples
///
/// ```ignore
/// use kernel::prelude::*;
///
/// // Declares a `#[vtable]` trait
/// #[vtable]
/// pub trait Operations: Send + Sync + Sized {
///     fn foo(&self) -> Result<()> {
///         Err(EINVAL)
///     }
///
///     fn bar(&self) -> Result<()> {
///         Err(EINVAL)
///     }
/// }
///
/// struct Foo;
///
/// // Implements the `#[vtable]` trait
/// #[vtable]
/// impl Operations for Foo {
///     fn foo(&self) -> Result<()> {
/// #        Err(EINVAL)
///         /* ... */
///     }
/// }
///
/// assert_eq!(<Foo as Operations>::HAS_FOO, true);
/// assert_eq!(<Foo as Operations>::HAS_BAR, false);
/// ```
#[proc_macro_attribute]
pub fn vtable(attr: TokenStream, ts: TokenStream) -> TokenStream {
    vtable::vtable(attr, ts)
}
