//! # wll-macros
//!
//! Procedural macros for `wll`.

extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, Meta, NestedMeta, ReturnType, Signature};

/// Mark the function as the initialization function for Wolfram LibraryLink.
///
/// **see also**: [Library Structure and Life Cycle: Initialization](https://reference.wolfram.com/language/LibraryLink/tutorial/LibraryStructure.html#280210622)
#[proc_macro_attribute]
pub fn wll_setup(_args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemFn);
    let funcall = match &ast.sig {
        Signature {
            ident: id,
            inputs: params,
            asyncness: None,
            unsafety: None,
            variadic: None,
            output: ret,
            ..
        } if params.is_empty() => {
            if let ReturnType::Type(_, ty) = ret {
                quote_spanned! { ty.span() =>
                    if let ::std::result::Result::Err(e) = #id() {
                        return e.to_raw_error();
                    }
                }
            } else {
                quote_spanned! { id.span() => #id() }
            }
        }
        sig => syn::Error::new(sig.span(), "Invalid function signature!").to_compile_error(),
    };
    (quote! {
        #[inline(always)]
        #ast
        #[no_mangle]
        pub extern "C" fn WolframLibrary_initialize(
                data: ::std::option::Option<::wll_sys::WolframLibraryData>
            ) -> ::wll_sys::errcode_t {
            if let ::std::result::Result::Err(e) = ::wll::global::initialize_lib_data(data) {
                e.to_raw_error()
            } else {
                #funcall;
                ::wll_sys::LIBRARY_NO_ERROR
            }
        }
        #[no_mangle]
        pub extern "C" fn WolframLibrary_getVersion() -> ::wll_sys::mint {
            ::wll_sys::WolframLibraryVersion as ::wll_sys::mint
        }
    })
    .into()
}

/// Mark the function as the uninitialization function for Wolfram LibraryLink.
///
/// **see also**: [Library Structure and Life Cycle: Uninitialization](https://reference.wolfram.com/language/LibraryLink/tutorial/LibraryStructure.html#441777402)
#[proc_macro_attribute]
pub fn wll_teardown(_args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemFn);
    let funcall = match &ast.sig {
        Signature {
            ident: id,
            inputs: params,
            asyncness: None,
            unsafety: None,
            variadic: None,
            output: ReturnType::Default,
            ..
        } if params.is_empty() => quote_spanned! { id.span() => #id() },
        sig => syn::Error::new(sig.span(), "Invalid function signature!").to_compile_error(),
    };
    (quote! {
        #[inline(always)]
        #ast
        #[no_mangle]
        pub extern "C" fn WolframLibrary_uninitialize(
                _: ::std::option::Option<::wll_sys::WolframLibraryData>
            ) {
                #funcall;
            }
    })
    .into()
}

#[proc_macro_attribute]
pub fn wll_export(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as ItemFn);
    let Signature { ident: funame, .. } = &input.sig;
    let exportname = match args.len() {
        0 => format_ident!("wll_{}", funame).into_token_stream(),
        _ => get_export_name_from_meta(&args[0]),
    };
    (quote! {
        #[inline]
        #input
        #[no_mangle]
        pub unsafe extern "C" fn #exportname(
            lib_data: ::std::option::Option<::wll_sys::WolframLibraryData>,
            argc: ::wll_sys::mint,
            args: *const ::wll_sys::MArgument,
            res: ::wll_sys::MArgument,
        ) -> ::wll_sys::errcode_t {
            // TODO
        }
    })
    .into()
}

fn get_export_name_from_meta(meta: &NestedMeta) -> proc_macro2::TokenStream {
    match meta {
        NestedMeta::Meta(Meta::Path(path)) => {
            if let Some(ident) = path.get_ident() {
                ident.to_token_stream()
            } else {
                syn::Error::new(path.span(), "expected identifier for export name.")
                    .to_compile_error()
            }
        }
        NestedMeta::Lit(Lit::Str(str)) => match str.parse::<syn::Ident>() {
            Ok(ident) => ident.into_token_stream(),
            Err(e) => e.to_compile_error(),
        },
        other => {
            syn::Error::new(other.span(), "expected identifier for export name.").to_compile_error()
        }
    }
}
