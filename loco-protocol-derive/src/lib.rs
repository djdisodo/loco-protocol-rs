extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Ident, DeriveInput};

#[proc_macro_derive(NamedCommandPair)]
pub fn name_command_pair(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let name = derive_input.ident;
    let name_uppercase = Ident::new(&name.to_string().to_uppercase(), name.span());

    let token_stream = quote! {
        impl loco_protocol::command::NamedCommand for #name {
            const NAME: &'static str = &stringify!(#name_uppercase);
        }

        impl<T: loco_protocol::command::PairedCommandRequest<#name>> loco_protocol::command::NamedCommand for T {
            const NAME: &'static str = &stringify!(#name_uppercase);
        }

        impl<T: loco_protocol::command::PairedCommandResponse<#name>> loco_protocol::command::NamedCommand for T {
            const NAME: &'static str = &stringify!(#name_uppercase);
        }
    };

    token_stream.into()
}

#[proc_macro_derive(NamedCommand)]
pub fn name_command(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let name = derive_input.ident;
    let name_uppercase = Ident::new(&name.to_string().to_uppercase(), name.span());

    let token_stream = quote! {
        impl loco_protocol::command::NamedCommand for #name {
            const NAME: &'static str = &stringify!(#name_uppercase);
        }
    };

    token_stream.into()
}