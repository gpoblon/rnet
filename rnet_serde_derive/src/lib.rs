// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2020 Gaëtan Poblon <gaetan.poblon@gmail.com>

extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(RnetSerde)]
pub fn rnet_serde_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        use crate::{ PayloadKind, WRnetSerde };

        impl RnetSerde for #name {
            fn new() -> Self {
                Default::default()
            }

            fn from_bytes<'de>(bytes: &'de [u8]) -> Self
            where {
                bincode::deserialize::<Self>(&bytes[1..]).expect("error: FromBytes from RnetSerde")
            }

            fn as_bytes(&self) -> Vec<u8>
            where Self: Serialize
            {
                let payload = WRnetSerde::new(self.clone(), PayloadKind::#name as u8);
                bincode::serialize(&payload).expect("error: AsBytes from RnetSerde")
            }

            fn action(datagram: &[u8])
            where Self: std::fmt::Debug + Sized
            {
                let ser: Self = Self::from_bytes(datagram);
                println!("(default){:?} received, content: {:#?}", PayloadKind::#name, ser);
            }
        }
    };
    gen.into()
}