use proc_macro2::{ Ident, TokenStream, Span};
use quote::quote;
use syn::{ Attribute, Data, Result, token };
use crate::derives::{
  entity::{ set_getters, set_setters },
  utils::{ get_table_name, get_table_list, set_ident },
};

pub fn handle_entity_model(data: Data, attrs: Vec<Attribute>) -> Result<TokenStream> {
  let table_name: String = get_table_name(attrs);
  let table_list: Vec<TableField> = get_table_list(data);
  let ts_entitys_to_models: TokenStream = ts_entity_to_model(table_list.clone());
  let ts_entitys: TokenStream = set_entitys(table_list);

  Ok(quote! {
    #ts_entitys_to_models

    #ts_entitys

    #[automatically_derived]
    impl mydb_sqlx::BaseEntity for Entity {
      fn table_name() -> &'static str {
        #table_name
      }
    }
  })
}


pub const NUMBER_TYPES: &[&str] = &[
    "i8", "u8", "i16", "u16", "i32", "u32",
    "i64", "u64", "i128", "u128", "isize", "usize"
];

#[derive(Debug, Clone, Default)]
pub struct MyField {
  pub field_type: Option<Ident>,
  pub lt: Option<String>,
  pub children: Option<Box<MyField>>,
  pub gt: Option<String>,
}

impl MyField {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct TableField {
  pub name: String,
  pub field_type: TokenStream,
  pub field_span: String,
  pub exist: bool,
  pub my_field: MyField,
}

pub fn set_entitys (table_list: Vec<TableField>) -> TokenStream {
  Entitys::new(table_list.clone()).set_entitys()
  .build(
    set_getters(table_list.clone()),
    set_setters(table_list),
  )
}

#[derive(Debug, Clone, Default)]
pub struct Entitys {
  pub data: Vec<TableField>,
  pub ts_entitys: Vec<TokenStream>,
}

impl Entitys {
  pub fn new(data: Vec<TableField>) -> Self {
    Entitys {
      data,
      ts_entitys: Vec::new(),
    }
  }

  pub fn set_entitys(mut self) -> Self {
    for field in &self.data {
      if field.exist {
        let name = set_ident(&field.name);
        let field_type = &field.field_type;
        self.ts_entitys.push(
          quote! { pub #name: #field_type, }
        );
      }
    }
    self
  }

  pub fn build (self, ts_getters: Vec<TokenStream>, ts_setters: Vec<TokenStream>) -> TokenStream {
    let ts_entitys = self.ts_entitys;
    quote! {
      #[derive(Debug, Clone, Default, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
      pub struct Entity {
        #(#ts_entitys)*
      }

      #[automatically_derived]
      impl Entity {
        #(#ts_getters)*
        #(#ts_setters)*
      }
    }
  }
}


/// Entity to Model
/// ### Usage (add TableData Automatic generation Entity)
/// ```
/// use serde::{Deserialize, Serialize};
/// use sqlx::FromRow;
/// use mydb_macros::TableData;
/// 
/// #[derive(TableData, Debug, Clone, Default, FromRow, Deserialize, Serialize)]
/// #[mydb(table_name = "sys_user")]
/// pub struct Model {
///   pub username: String,
///   pub password: String,
///   #[table_field(exist)]
///   pub role_ids: Vec<u64>,
/// }
/// 
/// let sys_user_entity: Entity = Entity {
///   username: format!("admin"),
///   password: format!("96e79218965eb72c92a549dd5a330112"),
///   ..Default::default()
/// };
/// 
/// let m: Model = sys_user_entity.into();
/// m.role_ids = vec![1, 2];
/// println!("model : {:#?}", m);
/// ```
pub fn ts_entity_to_model(data: Vec<TableField>) -> TokenStream {
  let mut model: Vec<TokenStream> = Vec::new();
  for field in data {
    if field.exist {
      let name = Ident::new(&field.name, Span::call_site());
      model.push(
        quote! { #name: entity.#name, }
      );
    }
  }
  quote! {
    #[automatically_derived]
    impl From<Entity> for Model {
      fn from(entity: Entity) -> Model {
        Model {
          #(#model)*
          ..Default::default()
        }
      }
    }
  }
}