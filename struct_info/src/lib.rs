use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, Field};

#[proc_macro_derive(StructInfoDerive, attributes(StructInfoName))]
pub fn struct_info_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate

    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_struct_info(&ast)
}

/// Tries to convert the expression to an identifier. This only works if the expression is a
/// string.
///
/// # Arguments
/// * `expr` - The expression to convert.
fn try_expr_to_string(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Lit(l) => match &l.lit {
            syn::Lit::Str(s) => {
                let s = s.value();
                Some(s)
            }
            _ => None,
        },
        _ => None,
    }
}

/// Tries to return the attributed field name if one is defined.
///
/// # Arguments
/// * `field` - The field whose attributes will be checked.
fn get_attribute_name(field: &Field) -> Option<String> {
    for attribute in field.attrs.iter() {
        match &attribute.meta {
            syn::Meta::NameValue(name_value) => {
                if name_value.path.is_ident("StructInfoName") {
                    match try_expr_to_string(&name_value.value) {
                        Some(name) => {
                            return Some(name);
                        }
                        None => return None,
                    }
                }
            }
            _ => {}
        }
    }

    None
}

/// The implementation for the struct info derive macro, that automatically generates the
/// StructInfo trait implementation from the parsed struct.
///
/// # Arguments
/// `ast` - The parsed struct
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
                let attribute_name = get_attribute_name(field);
                let name = field.ident.as_ref().unwrap();

                let attribute_name = match attribute_name {
                    Some(a) => a,
                    _ => name.to_string(),
                };

                field_names.extend(quote!(
                    #index => #attribute_name,
                ));

                field_parsing.extend(quote!(
                    #index => {
                        self.#name.csv_parse(s)?;
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
