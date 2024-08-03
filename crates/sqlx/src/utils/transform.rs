use serde::Serialize;
use std::collections::{ BTreeMap, HashMap };

pub fn struct_to_hashmap<T: Serialize>(data: T) -> HashMap<String, serde_json::Value> {
  let json_data = serde_json::to_string(&data).unwrap();
  let hashmap: HashMap<String, serde_json::Value> = serde_json::from_str(&json_data).unwrap();
  hashmap
}

pub fn struct_to_btreemap<T: Serialize>(data: T) -> BTreeMap<String, serde_json::Value> {
  let json_data = serde_json::to_string(&data).unwrap();
  let hashmap: BTreeMap<String, serde_json::Value> = serde_json::from_str(&json_data).unwrap();
  hashmap
}
