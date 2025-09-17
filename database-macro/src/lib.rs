extern crate proc_macro2;
use std::io::Read;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};
use indexmap::IndexMap;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::{Statement, DataType};

#[proc_macro]
pub fn generate_structs_from_sql(input: TokenStream) -> TokenStream {
    let file_path_str = parse_macro_input!(input as LitStr);
    let mut file = std::fs::File::open(file_path_str.value().as_str())
        .map_err(|e| {
            let err = syn::Error::new(
                file_path_str.span(), format!("Cannot open file: {e}")
            );
            return err.to_compile_error();
        }).unwrap();
    let mut sql_content: String = String::new();
    file.read_to_string(&mut sql_content).map_err(|e| {
        let err = syn::Error::new(
            file_path_str.span(), format!("Cannot read file: {e}")
        );
        return err.to_compile_error();
    }).unwrap();

    let mut structs = Vec::new();
    
    if let Some(parsed_sql) = parse_sql(&sql_content) {
        for (struct_name, fields) in parsed_sql {
            let mut field_def = Vec::new();
            let mut field_idents = Vec::new();
            let mut impl_def = Vec::new();
            let mut impl_param_def = Vec::new();

            let struct_ident = syn::Ident::new(&struct_name, proc_macro2::Span::call_site());
            for (field_name, type_name) in fields {
                let field_ident = syn::Ident::new(&field_name, proc_macro2::Span::call_site());
                let field_type = match type_name.as_str() {
                    "String" => quote! { String },
                    "i64" => quote! { i64 },
                    "i32" => quote! { i32 },
                    "i16" => quote! { i16 },
                    "bool" => quote! { bool },
                    "f32" => quote! { f32 },
                    "f64" => quote! { f64 },
                    "Vec<u8>" => quote! { Vec<u8> },
                    "Vec<T>" => quote! { Vec<T> },
                    _ => panic!("Unsupported type: {}", type_name)
                };
                field_def.push(quote! {
                    pub #field_ident: #field_type
                });
                impl_param_def.push(quote! {
                    #field_ident: #field_type
                });
                field_idents.push(quote! {
                    #field_ident
                });
            }

            impl_def.push(quote! {
                impl #struct_ident {
                    pub fn new(#(#impl_param_def),*) -> Self {
                        Self {
                            #(#field_idents),*
                        }
                    }
                }
            });

            let struct_def = quote! {
                #[derive(Debug)]
                pub struct #struct_ident {
                    #(#field_def),*
                }

                #(#impl_def)*
            };

            structs.push(struct_def);
        }
    }
    
    let expanded = quote! {
        #(#structs)*
    };

    expanded.into()
}

fn parse_sql(content: &String) -> Option<IndexMap<String, IndexMap<String, String>>> {
    let dialect = GenericDialect {};
    let ast = Parser::parse_sql(&dialect, content);
    let mut res = IndexMap::new();

    match ast {
        Ok(statements) => {
            for stmt in statements {
                if let Statement::CreateTable(t)= stmt {
                    let n = t.name.clone().to_string();
                    let table_name_chars = n.as_str().chars();
                    let mut chars = table_name_chars.clone();
                    let table_name = match chars.next() {
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                        None => String::new(),
                    };
                    let mut fields = IndexMap::new();
                    for column in t.columns {
                        // let table_name = t.name.to_string();
                        let name = column.name.to_string();
                        let data_type = column.data_type;
                        let rust_type = match data_type {
                            DataType::Varchar(_)
                            | DataType::Text
                            | DataType::Char(_) => "String".to_string(),
                            DataType::Int(_) | DataType::Integer(_) => "i32".to_string(),
                            DataType::BigInt(_) => "i64".to_string(),
                            DataType::SmallInt(_) => "i16".to_string(),
                            DataType::Float(_)
                            | DataType::Real => "f32".to_string(),
                            DataType::Double(_)
                            | DataType::DoublePrecision => "f64".to_string(),
                            DataType::Bool
                            | DataType::Boolean => "bool".to_string(),
                            DataType::Blob(_)
                            | DataType::Bytea => "Vec<u8>".to_string(),
                            DataType::Array(_) => "Vec<T>".to_string(),
                            _ => panic!("Unknow type: {}", data_type)
                        };
                        fields.insert(name, rust_type);
                        res.insert(table_name.clone(), fields.clone());
                    }
                    // panic!("{:?}", res.values());
                }
            }
            return Some(res)
        },
        Err(_) => return None
    }
}