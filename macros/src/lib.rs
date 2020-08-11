extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, ReturnType, Signature};

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
            if let ReturnType::Type(_, _) = ret {
                quote! { #id() }
            } else {
                quote! { #id() }
            }
        }
        _ => panic!("Invalid function signature!"),
    };
    (quote! {
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
        } if params.is_empty() => quote! { #id() },
        _ => panic!("Invalid function signature!"),
    };
    (quote! {
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
