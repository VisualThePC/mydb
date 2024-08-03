use proc_macro2::{ Ident, Span, TokenStream };
use quote::quote;
use crate::derives::{
  entity::TableField,
  utils::set_ident 
};

pub fn set_getters(table_list: Vec<TableField>) -> Vec<TokenStream> {
  Getters::new(table_list).set_getters()
}

#[derive(Debug, Clone, Default)]
pub struct Getters {
  pub data: Vec<TableField>,
  pub ts_getters: Vec<TokenStream>,
}

impl Getters {
  pub fn new(data: Vec<TableField>) -> Self {
    Getters {
      data,
      ts_getters: Vec::new(),
    }
  }

  pub fn set_getters(&mut self) -> Vec<TokenStream> {
    for field in &self.data {
      if field.exist {
        let get_name = set_ident(&format!("get_{}", &field.name));
        let name = set_ident(&field.name);
        let field_type = &field.field_type;
        if field.field_span.contains("DateTime") {
          self.ts_getters.push(
            quote! {
              pub fn #get_name(&self) -> #field_type {
                self.#name
              }
            }
          );
        } else {
          self.ts_getters.push(
            quote! {
              pub fn #get_name(&self) -> #field_type {
                self.#name.clone()
              }
            }
          );
        }
      }
    }
    self.ts_getters.clone()
  }
}
