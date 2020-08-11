extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn wll_setup(_args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemFn);
    (quote! {
        #ast
        #[no_mangle]
        pub extern "C" fn WolframLibrary_initialize(
                data: std::option::Option<::wll_sys::WolframLibraryData>
            ) -> ::wll_sys::errcode_t {
            if let Err(e) = ::wll::global::initialize_lib_data(data) {
                e.to_raw_error()
            } else {
                ::wll_sys::LIBRARY_NO_ERROR
            }
        }
        #[no_mangle]
        pub extern "C" fn WolframLibrary_getVersion() -> ::wll_sys::mint {
            ::wll_sys::WolframLibraryVersion
        }
    })
    .into()
}
