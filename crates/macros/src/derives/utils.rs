use proc_macro2::{TokenTree, Ident, Span, TokenStream};
use syn::{
  Attribute, Data, Fields, Field, GenericArgument, Meta, PathArguments, Type, TypePath
};
use quote::quote;

use crate::derives::entity::{ TableField, MyField };

pub fn get_table_list(data: Data) -> Vec<TableField> {
  let mut table_list: Vec<TableField> = vec![];
  match data {
    Data::Struct(data_struct) => {
      match data_struct.fields {
        Fields::Named(fields) => {
          for field in fields.named {
            let name = get_field_name(&field);
            let mut field_type = TokenStream::new();
            let mut field_span = "".to_string();
            let mut my_field: MyField = MyField::new();
            let mut exist = true;

            field.attrs.iter()
            .filter(| attr | attr.path().is_ident("table_field"))
            .for_each(|attr| {

              if let Meta::List(lits) = &attr.meta {
                let mut table_field_content = format!("");
                let _ = &lits.tokens.clone().into_iter().for_each(| ts | {
                  table_field_content += &ts.span().source_text().unwrap_or(format!(""));
                });
                table_field_content = table_field_content.replace(" ", "");
                let exist_vec: &Vec<&str> = &table_field_content.split(',').collect();
                if exist_vec.contains(&"exist=false") {
                  exist = false;
                }
              }
            });

            if let Type::Path(_ty) = &field.ty {
              my_field = recursive_print_ident(_ty);
              field_type = my_field_ts(my_field.clone());
              field_span = my_field_span(my_field.clone());
            }
            table_list.push( TableField {
              name,
              field_type,
              exist,
              my_field,
              field_span,
            })
          }
        },
        _ => ()
      }
    },
    _ => ()
  }
  table_list
}

pub fn get_field_name(field: &Field) -> String {
  if let Some(ident) = &field.ident {
    if let Some(text) = ident.span().source_text() {
      return text;
    }
  }
  format!("")
}

pub fn get_table_name(attrs: Vec<Attribute>) -> String {
  let mut table_name = format!("");
  attrs
  .iter()
  .filter(|attr| attr.path().is_ident("mydb"))
  .into_iter()
  .for_each(| attr | {
    if let Meta::List(lits) = &attr.meta {
      let _ = &lits.tokens.clone().into_iter().for_each(| ts | {
        if let TokenTree::Literal(l_name) = ts {
          if l_name.to_string().len() > 0 {
            table_name = l_name.to_string().replace('"', "");
          } else {
            eprintln!("no setting table_name")
          }
        }
      });
    }
  });
  table_name
}

pub fn set_ident(name: &String) -> Ident {
  Ident::new(&name, Span::call_site())
}

pub fn recursive_print_ident(type_path: &TypePath) -> MyField {
  let mut my_field: MyField = MyField::new();
  for segment in &type_path.path.segments {
    my_field.field_type = Some(segment.ident.clone());
    match &segment.arguments {
        PathArguments::AngleBracketed(args) => {
          my_field.lt = Some("<".to_string());
            for arg in &args.args {
                if let GenericArgument::Type(ty) = arg {
                    if let Type::Path(inner_path) = ty {
                      my_field.children = Some(Box::new(recursive_print_ident(inner_path)));
                    }
                }
            }
            my_field.gt = Some(">".to_string());
        }
        _ => {}
    }
  }
  my_field
}

pub fn my_field_ts(my_field: MyField) -> TokenStream {
  let mut ts: Vec<TokenStream> = Vec::new();

  match my_field.field_type {
    Some(_field_type) => ts.push(quote!(#_field_type)),
    None => (),
  };

  match my_field.lt {
    Some(_) => ts.push(quote!(<)),
    None => (),
  };

  match my_field.children {
    Some(children) => {
      let text = my_field_ts(*children);
      ts.push(text)
    },
    None => (),
  };

  match my_field.gt {
    Some(_)  => ts.push(quote!(>)),
    None => (),
  };

  quote!{ #(#ts)* }
}


pub fn my_field_span(my_field: MyField) -> String {
  let mut s = "".to_string();

  match my_field.field_type {
    Some(field_type) => s += &field_type.to_string(),
    None => (),
  };

  match my_field.lt {
    Some(_) => s += "<",
    None => (),
  };

  match my_field.children {
    Some(children) => {
      s += &my_field_span(*children);
    },
    None => (),
  };


  match my_field.gt {
    Some(_)  => s += ">",
    None => (),
  };

  s
}