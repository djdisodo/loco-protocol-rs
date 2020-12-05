extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Ident, DeriveInput, Error};
use syn::parse::ParseBuffer;
use syn::Token;

struct LocoPacketPair(Ident, Ident);

impl syn::parse::Parse for LocoPacketPair {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let content;
        syn::parenthesized!(content in input);
        let type1 = content.parse()?;
        content.parse::<Token![,]>()?;
        let type2 = content.parse()?;
        Ok(LocoPacketPair(type1, type2))
    }
}

#[proc_macro_derive(LocoPacketPair, attributes(loco_packet_pair))]
pub fn derive_loco_packet_pair(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let attribute = derive_input.attrs.iter().filter(
        |a| a.path.segments.len() == 1 && a.path.segments[0].ident == "loco_packet_pair"
    ).nth(0).expect("attribute loco_packet_pair(request, response)");

    let command_pair: LocoPacketPair = syn::parse2(attribute.tokens.clone()).expect("Invalid loco_packet_pair attribute!");

    let name = derive_input.ident;
    let request = command_pair.0;
    let response = command_pair.1;
    let name_uppercase = Ident::new(&name.to_string().to_uppercase(), name.span());

    let token_stream = quote! {

        impl loco_protocol::LocoPacket for #request {
            const NAME: &'static str = &stringify!(#name_uppercase);
        }

        impl loco_protocol::LocoRequest for #request {}

        impl loco_protocol::LocoPacket for #response {
            const NAME: &'static str = &stringify!(#name_uppercase);
        }

        impl loco_protocol::LocoResponse for #response {}

        impl loco_protocol::command::LocoPacketPair<#request, #response> for #name {
            const NAME: &'static str = &stringify!(#name_uppercase);
        }
    };

    token_stream.into()
}

#[proc_macro_derive(LocoPacket)]
pub fn derive_loco_packet(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let name = derive_input.ident;
    let name_uppercase = Ident::new(&name.to_string().to_uppercase(), name.span());

    let token_stream = quote! {
        impl loco_protocol::LocoPacket for #name {
            const NAME: &'static str = &stringify!(#name_uppercase);
        }
    };

    token_stream.into()
}

#[proc_macro_derive(LocoRequest)]
pub fn derive_loco_request(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let name = derive_input.ident;
    let name_uppercase = Ident::new(&name.to_string().to_uppercase(), name.span());

    let token_stream = quote! {
        impl loco_protocol::LocoPacket for #name {
            const NAME: &'static str = &stringify!(#name_uppercase);
        }

        impl loco_protocol::LocoRequest for #name {}
    };

    token_stream.into()
}

#[proc_macro_derive(LocoResponse)]
pub fn derive_loco_response(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let name = derive_input.ident;
    let name_uppercase = Ident::new(&name.to_string().to_uppercase(), name.span());

    let token_stream = quote! {
        impl loco_protocol::LocoPacket for #name {
            const NAME: &'static str = &stringify!(#name_uppercase);
        }

        impl loco_protocol::LocoRequest for #name {}
    };

    token_stream.into()
}