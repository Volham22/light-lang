extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Ident};

fn expand_debug_info_struct(input: &DeriveInput, strct: &DataStruct) -> TokenStream {
    let fields = strct.fields.iter();
    let ident = &input.ident;
    let vis = &input.vis;

    let expand = if !strct.fields.is_empty() {
        quote! {
            #[derive(Clone)]
            #vis struct #ident {
                #(#fields),*,
                pub line: usize,
                pub column: usize,
                pub filename: String,
            }

            impl LineDebugInfo for #ident {
                fn line(&self) -> usize {
                    self.line
                }

                fn column(&self) -> usize {
                    self.column
                }

                fn file_name<'a>(&'a self) -> &'a str {
                    &self.filename
                }
            }
        }
    } else {
        quote! {
            #[derive(Clone)]
            #vis struct #ident {
                pub line: usize,
                pub column: usize,
                pub filename: String,
            }

            impl LineDebugInfo for #ident {
                fn line(&self) -> usize {
                    self.line
                }

                fn column(&self) -> usize {
                    self.column
                }

                fn file_name<'a>(&'a self) -> &'a str {
                    &self.filename
                }
            }
        }
    };

    TokenStream::from(expand)
}

fn expand_debug_info_enum(input: &DeriveInput, en: &DataEnum) -> TokenStream {
    let ident = &input.ident;
    let mut arms = Vec::new();

    for v in &en.variants {
        let v_ident = &v.ident;
        let f: Vec<Ident> = v
            .fields
            .iter()
            .enumerate()
            .map(|(i, _)| Ident::new(format!("enum_variant_{}", i).as_str(), Span::call_site()))
            .collect();
        let args = quote! {
            #(#f),*
        };

        let arm = if let Some(first_name) = f.first() {
            quote! {
                #ident::#v_ident(#args) => #first_name
            }
        } else {
            quote! {
                #ident::#v_ident => 0usize
            }
        };
        arms.push(arm);
    }

    let expand = quote! {
        #[derive(Clone)]
        #input

        impl LineDebugInfo for #ident {
            fn line(&self) -> usize {
                match self {
                    #(#arms .line()),*
                }
            }

            fn column(&self) -> usize {
                match self {
                    #(#arms .column()),*
                }
            }

            fn file_name<'a>(&'a self) -> &'a str {
                match self {
                    #(#arms .file_name()),*
                }
            }
        }
    };

    TokenStream::from(expand)
}

#[proc_macro_attribute]
pub fn line_debug_info(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    match input.data {
        Data::Struct(ref strct) => expand_debug_info_struct(&input, &strct),
        Data::Enum(ref en) => expand_debug_info_enum(&input, &en),
        Data::Union(_) => panic!("Derive LineDebugInfo is not valid on union"),
    }
}
