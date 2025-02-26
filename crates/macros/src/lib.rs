mod build_macro;
mod implement;
mod implement_macro;

use build_macro::*;
use gen::*;
use implement_macro::*;
use quote::*;
use reader::*;
use syn::parse_macro_input;

/// A macro for generating WinRT modules to a .rs file at build time.
///
/// This macro can be used to import WinRT APIs from any Windows metadata (winmd) file.
/// It is only intended for use from a crate's build.rs script.
///
/// The macro generates a single `build` function which can be used in build scripts
/// to generate the WinRT bindings. After using the `build` macro, call the
/// generated `build` function somewhere in the build.rs script's main function.
///
/// # Usage
/// To use, you must then specify which types you want to use. These
/// follow the same convention as Rust `use` paths. Types know which other types they depend on so
/// `build` will generate any other WinRT types needed for the specified type to work.
///
/// # Example
/// The following `build!` generates all types inside of the `Microsoft::AI::MachineLearning`
/// namespace.
///
/// ```rust,ignore
/// build!(
///     Microsoft::AI::MachineLearning::*
/// );
/// ```
#[proc_macro]
pub fn build(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(stream as BuildMacro);
    gen_build().as_str().parse().unwrap()
}

#[proc_macro]
pub fn generate(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(stream as BuildMacro);

    let mut tokens = String::new();
    tokens.push_str("r#\"");
    tokens.push_str(&gen_source_tree().into_string());
    tokens.push_str("\"#");
    tokens.parse().unwrap()
}

/// Rust structs can use the [`macro@implement`] attribute macro to implement entire WinRT
/// classes or any combination of existing COM and WinRT interfaces.
///
/// If the attribute [`proc_macro::TokenStream`] contains the name of a WinRT class then all
/// of its interfaces are implemented. Otherwise, whatever interfaces are contained within
/// the attribute TokenStream are implemented.
#[proc_macro_attribute]
pub fn implement(
    attribute: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    implement::gen(attribute, input)
}
