use proc_macro2::{ Ident, Span, TokenStream, Punct };
use quote::quote;
use crate::derives::{
  entity::MyField,
  utils::{ set_ident, get_field_name, recursive_print_ident, my_field_ts }
};
use syn::{ Data, Fields, Result, Type };

pub fn get_token_stream_setters_impl(data: Data, ident: Ident) -> TokenStream {
  let imp_name: Ident = ident;
  let get_vec: Vec<SetItem> = get_list(data);
  let token_stream_setters: Vec<TokenStream> = SetData::new(get_vec).build();

  quote! {
    impl #imp_name {
      #(#token_stream_setters)*
    }
  }
}

pub fn get_token_stream_array(data: Data) -> Vec<TokenStream> {
  let get_vec: Vec<SetItem> = get_list(data);
  SetData::new(get_vec).build()
}

#[derive(Debug, Clone, Default)]
pub struct SetData {
  pub data: Vec<SetItem>,
  pub ts_setters: Vec<TokenStream>,
}

#[derive(Debug, Clone, Default)]
pub struct SetItem {
  pub name: String,
  pub field_type: TokenStream,
}

impl SetData {
  pub fn new(data: Vec<SetItem>) -> Self {
    SetData {
      data,
      ts_setters: Vec::new(),
    }
  }

  pub fn build(&mut self) -> Vec<TokenStream> {
    for field in &self.data {
      let set_name = set_ident(&format!("set_{}", &field.name));
      let name = set_ident(&field.name);
      let field_type = &field.field_type;
      self.ts_setters.push(
        quote! {
          pub fn #set_name(&mut self, param: #field_type) -> &mut Self {
            self.#name = param;
            self
          }
        }
      );
    }
    self.ts_setters.clone()
  }
}


pub fn get_list(data: Data) -> Vec<SetItem> {
  let mut sets: Vec<SetItem> = vec![];
  match data {
    Data::Struct(data_struct) => {
      match data_struct.fields {
        Fields::Named(fields) => {
          for field in fields.named {
            let name = get_field_name(&field);
            let mut field_type = TokenStream::new();
            let mut my_field: MyField = MyField::new();
            
            if let Type::Path(_ty) = &field.ty {
              my_field = recursive_print_ident(_ty);
              field_type = my_field_ts(my_field.clone());
            }
            sets.push(SetItem {
              name,
              field_type,
            })
          }
        },
        _ => ()
      }
    },
    _ => ()
  }
  sets
}