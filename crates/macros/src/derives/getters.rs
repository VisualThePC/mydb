use proc_macro2::{ Ident, Span, TokenStream, Punct };
use quote::quote;
use crate::derives::{
  entity::MyField,
  utils::{ set_ident, get_field_name, recursive_print_ident, my_field_ts, my_field_span }
};
use syn::{ Data, Fields, Result, Type };

pub fn get_token_stream_getters_impl(data: Data, ident: Ident) -> TokenStream {
  let imp_name: Ident = ident;
  let get_vec: Vec<GetItem> = get_list(data);
  let token_stream_getters: Vec<TokenStream> = GetData::new(get_vec).build();

  quote! {
    impl #imp_name {
      #(#token_stream_getters)*
    }
  }
}

pub fn get_token_stream_array(data: Data) -> Vec<TokenStream> {
  let get_vec: Vec<GetItem> = get_list(data);
  GetData::new(get_vec).build()
}

#[derive(Debug, Clone, Default)]
pub struct GetData {
  pub data: Vec<GetItem>,
  pub token_stream_getters: Vec<TokenStream>,
}

#[derive(Debug, Clone, Default)]
pub struct GetItem {
  pub name: String,
  pub field_type: TokenStream,
  pub field_span: String,
}


impl GetData {
  pub fn new(data: Vec<GetItem>) -> Self {
    GetData {
      data,
      token_stream_getters: Vec::new(),
    }
  }

  pub fn build(&mut self) -> Vec<TokenStream> {
    for field in &self.data {
      let get_name = set_ident(&format!("get_{}", &field.name));
      let name = set_ident(&field.name);
      let field_type = &field.field_type;
      if field.field_span.contains("DateTime") {
        self.token_stream_getters.push(
          quote! {
            pub fn #get_name(&self) -> #field_type {
              self.#name
            }
          }
        );
      } else {
        self.token_stream_getters.push(
          quote! {
            pub fn #get_name(&self) -> #field_type {
              self.#name.clone()
            }
          }
        );
      }
    }
    self.token_stream_getters.clone()
  }
}

pub fn get_list(data: Data) -> Vec<GetItem> {
  let mut gets: Vec<GetItem> = vec![];
  match data {
    Data::Struct(data_struct) => {
      match data_struct.fields {
        Fields::Named(fields) => {
          for field in fields.named {
            let name = get_field_name(&field);
            let mut field_type = TokenStream::new();
            let mut my_field: MyField = MyField::new();
            let mut field_span: String = String::new();
            
            if let Type::Path(_ty) = &field.ty {
              my_field = recursive_print_ident(_ty);
              field_type = my_field_ts(my_field.clone());
              field_span = my_field_span(my_field.clone());
            }
            gets.push(GetItem {
              name,
              field_type,
              field_span,
            })
          }
        },
        _ => ()
      }
    },
    _ => ()
  }
  gets
}