extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Ident, DeriveInput};

#[proc_macro_derive(NameCommandPair)]
pub fn name_command_pair(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let name = derive_input.ident;
    let name_uppercase = Ident::new(&name.to_string().to_uppercase(), name.span());

    let token_stream = quote! {
        impl crate::command::NamedCommand for #name {
            const NAME: &'static str = &stringify!(#name_uppercase);
        }

        impl<T: crate::command::PairedCommandRequest<#name>> crate::command::NamedCommand for T {
            const NAME: &'static str = &stringify!(#name_uppercase);
        }

        impl<T: crate::command::PairedCommandResponse<#name>> crate::command::NamedCommand for T {
            const NAME: &'static str = &stringify!(#name_uppercase);
        }
    };

    token_stream.into()
}

#[proc_macro_derive(NameCommand)]
pub fn name_command(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let name = derive_input.ident;
    let name_uppercase = Ident::new(&name.to_string().to_uppercase(), name.span());

    let token_stream = quote! {
        impl crate::command::NamedCommand for #name {
            const NAME: &'static str = &stringify!(#name_uppercase);
        }
    };

    token_stream.into()
}

#[proc_macro_derive(RequestCommand)]
pub fn request_command(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let name = derive_input.ident;

    let token_stream = quote! {
        impl crate::command::PairedCommandRequest for #name {}
    };

    token_stream.into()
}

#[proc_macro_derive(ResponseCommand)]
pub fn response_command(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let name = derive_input.ident;

    let token_stream = quote! {
        impl crate::command::PairedCommandResponse for #name {}
    };

    token_stream.into()
}