use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(ByteSerializable)]
pub fn derive_byte_serializable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let fields = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(f) => f.named,
            _ => panic!("Only named structs supported"),
        },
        _ => panic!("Only structs supported"),
    };

    let serialize_fields = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;

        quote! {
            <#ty as byteser::ByteSerializable>::byte_serialize(&self.#ident, out);
        }
    });

    let deserialize_fields = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;

        quote! {
            #ident: <#ty as byteser::ByteSerializable>::byte_deserialize(input)?
        }
    });

    let expanded = quote! {

        impl byteser::ByteSerializable for #name {

            fn byte_serialize(&self, out: &mut Vec<u8>) {

                #(
                    #serialize_fields
                )*
            }

            fn byte_deserialize(input: &mut &[u8]) -> Result<Self,String> {

                Ok(Self{

                    #(
                        #deserialize_fields,
                    )*

                })

            }

        }

    };

    TokenStream::from(expanded)
}