mod derives;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error};

#[proc_macro_derive(TableData, attributes(mydb, table_name, table_field))]
pub fn table_data_derive(input: TokenStream) -> TokenStream {
  let DeriveInput {
      ident, data, attrs, vis, generics,
  } = parse_macro_input!(input as DeriveInput);

  let ts = derives::entity::handle_entity_model(data, attrs)
  .unwrap_or_else(Error::into_compile_error);

  TokenStream::from(ts)
}

#[proc_macro_derive(Getters, attributes())]
pub fn getters_derive(input: TokenStream) -> TokenStream {
  let DeriveInput {
    ident, data, attrs, vis, generics,
} = parse_macro_input!(input as DeriveInput);
  let ts = derives::getters::get_token_stream_getters_impl(data, ident);
  TokenStream::from(ts)
}

#[proc_macro_derive(Setters, attributes())]
pub fn setters_derive(input: TokenStream) -> TokenStream {
  let DeriveInput {
    ident, data, attrs, vis, generics,
} = parse_macro_input!(input as DeriveInput);
  let ts = derives::setters::get_token_stream_setters_impl(data, ident);
  TokenStream::from(ts)
}

