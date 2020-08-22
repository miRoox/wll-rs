//! # wll-macros
//!
//! Procedural macros for `wll`.

extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, AttributeArgs, FnArg, ItemFn, Lit, LitInt, Meta, NestedMeta, ReturnType,
    Signature,
};

/// Mark the function as the initialization function for Wolfram LibraryLink.
///
/// **see also**: [Library Structure and Life Cycle: Initialization](https://reference.wolfram.com/language/LibraryLink/tutorial/LibraryStructure.html#280210622)
#[proc_macro_attribute]
pub fn setup(_args: TokenStream, input: TokenStream) -> TokenStream {
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
        sig => invalid_function_signature(sig.span()),
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
            ::wll_sys::WolframLibraryVersion
        }
    })
    .into()
}

/// Mark the function as the uninitialization function for Wolfram LibraryLink.
///
/// **see also**: [Library Structure and Life Cycle: Uninitialization](https://reference.wolfram.com/language/LibraryLink/tutorial/LibraryStructure.html#441777402)
#[proc_macro_attribute]
pub fn teardown(_args: TokenStream, input: TokenStream) -> TokenStream {
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
        sig => invalid_function_signature(sig.span()),
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

/// Export function for Wolfram LibraryLink.
///
/// *see also*: [Library Structure and Life Cycle: Functions, Arguments, and Results](https://reference.wolfram.com/language/LibraryLink/tutorial/LibraryStructure.html#606935091)
#[proc_macro_attribute]
pub fn export(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as ItemFn);
    let Signature {
        ident: funame,
        inputs: params,
        paren_token: sig_paren,
        ..
    } = &input.sig;
    let exportname = match args.len() {
        0 => format_ident!("wll_{}", funame).into_token_stream(),
        _ => get_export_name_from_meta(&args[0]),
    };
    let paramc = params.len();
    let paramclit = LitInt::new(&paramc.to_string(), sig_paren.span);
    let argvars = (0..paramc).map(|i| format_ident!("arg{:o}", i));
    let argvars2 = argvars.clone();
    let arggets = params.iter().enumerate().map(|(i, arg)| match arg {
        FnArg::Receiver(recr) => invalid_function_signature(recr.span()),
        FnArg::Typed(pty) => {
            let ty = &pty.ty;
            let inner = quote_spanned! { ty.span() =>
                <#ty>::try_get_arg(args.add(#i).read())
            };
            unwrap_errcode(inner, ty.span())
        }
    });
    let (funcall, setres) = match &input.sig {
        Signature {
            asyncness: None,
            variadic: None,
            output: ReturnType::Type(_, ty),
            ..
        } => {
            let inner = quote_spanned! { input.sig.span() =>
                #funame(#(#argvars2),*)
            };
            let setres = quote_spanned! { ty.span() =>
                match ret.try_set_arg(&res) {
                    ::std::result::Result::Ok(()) => ::wll_sys::LIBRARY_NO_ERROR,
                    ::std::result::Result::Err(err) => err.to_raw_error(),
                }
            };
            (unwrap_errcode(inner, ty.span()), setres)
        }
        sig => (
            invalid_function_signature(sig.span()),
            invalid_function_signature(sig.span()),
        ),
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
            use ::wll::adaptor::{MArgumentGetter, MArgumentSetter};
            let _lib_data = ::wll::global::LibDataLocalizer::new(lib_data);
            if argc != #paramclit {
                return ::wll_sys::LIBRARY_TYPE_ERROR;
            }
            #(let #argvars = #arggets;)*
            let ret = #funcall;
            #setres
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

fn unwrap_errcode(
    expr: proc_macro2::TokenStream,
    span: proc_macro2::Span,
) -> proc_macro2::TokenStream {
    quote_spanned! { span =>
        match #expr {
            ::std::result::Result::Ok(val) => val,
            ::std::result::Result::Err(err) => return err.to_raw_error(),
        }
    }
}

fn invalid_function_signature(span: proc_macro2::Span) -> proc_macro2::TokenStream {
    syn::Error::new(span, "invalid function signature!").to_compile_error()
}
