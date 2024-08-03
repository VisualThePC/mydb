use proc_macro2::{ Ident, Span, TokenStream };
use quote::quote;
use crate::derives::{
  entity::index::TableField,
  utils::set_ident
};

pub fn set_setters(table_list: Vec<TableField>) -> Vec<TokenStream> {
  Setters::new(table_list).set_setters()
}
#[derive(Debug, Clone, Default)]
pub struct Setters {
  pub data: Vec<TableField>,
  pub ts_setters: Vec<TokenStream>,
}

impl Setters {
  pub fn new(data: Vec<TableField>) -> Self {
    Setters {
      data,
      ts_setters: Vec::new(),
    }
  }

  pub fn set_setters(&mut self) -> Vec<TokenStream> {
    for field in &self.data {
      if field.exist {
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
    }
    self.ts_setters.clone()
  }
}

