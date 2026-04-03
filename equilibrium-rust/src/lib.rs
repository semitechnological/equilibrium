//! Ergonomic FFI exports for equilibrium
//!
//! The `#[ffi]` attribute automatically adds `#[no_mangle]` and `extern "C"`
//! to make functions callable from C and other languages via equilibrium.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Mark a function for C FFI export.
#[proc_macro_attribute]
pub fn ffi(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    
    let attrs = &input_fn.attrs;
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let block = &input_fn.block;
    
    let mut new_sig = sig.clone();
    new_sig.abi = Some(syn::parse_quote!(extern "C"));
    
    let expanded = quote! {
        #[no_mangle]
        #(#attrs)*
        #vis #new_sig #block
    };
    
    TokenStream::from(expanded)
}

/// Mark a struct for C FFI compatibility.
#[proc_macro_attribute]
pub fn ffi_struct(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = item.into();
    
    let expanded = quote! {
        #[repr(C)]
        #input
    };
    
    TokenStream::from(expanded)
}
