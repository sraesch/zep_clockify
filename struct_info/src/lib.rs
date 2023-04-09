use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(StructInfoDerive)]
pub fn struct_info_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate

    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_struct_info(&ast)
}

fn impl_struct_info(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // check that we have a struct
    let s = match &ast.data {
        syn::Data::Struct(s) => s,
        _ => {
            return TokenStream::new();
        }
    };

    // determine the number of fields
    let num_fields = s.fields.iter().count();

    // create iterator that crates the field entries for the names
    let mut field_names = quote!();
    let mut field_parsing = quote!();

    match &s.fields {
        syn::Fields::Named(named) => {
            for (index, field) in named.named.iter().enumerate() {
                let name = field.ident.as_ref().unwrap();
                field_names.extend(quote!(
                    #index => stringify!(#name),
                ));

                field_parsing.extend(quote!(
                    #index => {
                        self.#name = s.parse()?;
                    },
                ));
            }
        }
        _ => unimplemented!(),
    }

    // create final implementation
    let gen = quote! {
        impl StructInfo for #name {
            const NUM_FIELDS: usize = #num_fields;

            fn get_field_name(index: usize) -> &'static str {
                match index {
                    #field_names
                    _ => panic!("Index {} is out of range", index),
                }
            }

            fn parse_field(&mut self, index: usize, s: &str) -> Result<(), anyhow::Error> {
                match index {
                    #field_parsing
                    _ => panic!("Index {} is out of range", index),
                }

                Ok(())
            }
        }
    };
    gen.into()
}
