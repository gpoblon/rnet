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
        impl RnetSerde for #name {
            // fn new() -> Self {
            //     Default::default()
            // }

            fn as_ref(&self) -> &Self {
                &self
            }

            fn payload_from_bytes<'de>(bytes: &'de [u8]) -> Self
            where {
                bincode::deserialize::<Self>(&bytes[4..]).expect("error: FromBytes from RnetSerde")
            }

            fn prepare(&self, version: [u8;3]) -> Vec<u8>
            where Self: Serialize
            {
                // env!("CARGO_PKG_VERSION")
                let payload = WRnetSerde::new(self.as_ref(), PayloadKind::#name as u8, version);
                bincode::serialize(&payload).expect("error: AsBytes from RnetSerde")
            }

            fn debug(&self) {
                println!("(default){:?} received, content: {:?}", PayloadKind::#name, self);
            }

            fn action(datagram: &[u8])
            where Self: std::fmt::Debug + Sized
            {
                let ser: Self = Self::payload_from_bytes(datagram);
                ser.debug();
            }
        }
    };
    gen.into()
}