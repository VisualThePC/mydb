use std::marker::PhantomData;
use sqlx::mysql::MySqlRow;
use serde_derive::{ Serialize, Deserialize };
use sqlx::{ query, query_as, query_scalar };
use sqlx::{ Error, FromRow };
use sqlx::MySqlPool;
use sqlx_core::mysql::MySqlQueryResult;
use crate::transform::{ struct_to_hashmap, struct_to_btreemap };
use crate::BaseEntity;
use core::fmt::Debug;

pub static SELECT_FROM: &'static str = "SELECT * FROM";
pub static INSERT_INTO: &'static str = "INSERT INTO";
pub static UPDATE: &'static str = "UPDATE";
pub static DELETE_FROM: &'static str = "DELETE FROM";

pub fn mapper<T> (db: MySqlPool) -> BaseMapper<T> 
where 
T: for<'a> FromRow<'a, MySqlRow> + Send + Unpin + Debug + serde::Serialize + BaseEntity
{
  BaseMapper::<T>::new(db)
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PageData<T: BaseEntity> {
  pub total: i64,
  pub data: Vec<T>,
}

#[derive(Debug, Clone)]
pub struct BaseMapper<T: BaseEntity> {
  db: MySqlPool,
  table_name: &'static str,
  _marker: PhantomData<T>,
}

impl<T: for<'a> FromRow<'a, MySqlRow> + Send + Unpin + Debug + serde::Serialize + BaseEntity>
BaseMapper<T>
{
  pub fn new(db: MySqlPool) -> Self {
    Self {
      db,
      table_name: T::table_name(),
      _marker: PhantomData,
     }
  }

/******************************************** INSERT MAPPER ********************************************/
  pub async fn insert(self, entity: T) -> Result<MySqlQueryResult, Error> {
    let mut key: Vec<String>= Vec::new();
    let mut value: Vec<String> = Vec::new();

    for (k, v) in struct_to_hashmap(entity) {
      if v == "null" {
        continue;
      }
      key.push(k);
      value.push(format!("{}", v));
    };

    let sql_str = format!(
      "{} {} ( {} ) VALUES ( {} )",
      INSERT_INTO, self.table_name, key.join(", "), value.join(", ").replace('\"', "'")
    );

    println!("insert sql: {}", &sql_str);
    let mut pool = self.db.acquire().await?;
    match query(&sql_str).execute(&mut pool).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  pub async fn insert_batch_some_column(self, entity_vec: Vec<T>) -> Result<MySqlQueryResult, Error> {
    let mut keys: Vec<String> = Vec::new();
    let mut values: Vec<String> = Vec::new();
    let mut key_lock: bool = false;

    entity_vec
    .iter()
    .for_each(|entity| {
      let mut value: Vec<String> = Vec::new();
      for (k, v) in struct_to_btreemap(entity) {
        value.push(format!("{}", v));
        if key_lock == false {
          keys.push(k);
        }
      };
      values.push(format!("({})", value.join(", ").replace('\"', "'")));
      key_lock = true;
    });

    let sql_str = format!(
      "{} {} ({}) VALUES {}",
      INSERT_INTO, self.table_name, keys.join(", "), values.join(", ")
    );

    println!("insert_batch_some_column sql:{}", &sql_str);

    let mut pool = self.db.acquire().await?;
    match query(&sql_str).execute(&mut pool).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }
  /******************************************** INSERT MAPPER ********************************************/

  /******************************************** UPDATE MAPPER ********************************************/
  pub async fn update_by_id(self, entity: T) -> Result<MySqlQueryResult, Error> {
    let mut id: String = format!("");
    let mut set_value = Vec::new();

    for (k, v) in struct_to_hashmap(entity) {
      if k == "id" {
        id = format!("{}", v);
        continue;
      }

      if k == "create_time" || k == "update_time" || v.is_null() {
        continue;
      }
      
      set_value.push(format!("{} = {}", k, v));
    }

    let sql_str = format!(
      "{} {} SET {} WHERE id = {}",
      UPDATE, self.table_name, set_value.join(", ").replace('\"', "'"), id,
    );

    println!("update_by_id sql:{}", &sql_str);

    let mut pool = self.db.acquire().await?;
    match query(&sql_str).execute(&mut pool).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  pub async fn update(self, entity: T, wrapper: String) -> Result<MySqlQueryResult, Error> {
    let mut set_value = Vec::new();

    for (k, v) in struct_to_hashmap(entity) {
      if k == "id" || k == "create_time" || k == "update_time" || v.is_null() {
        continue;
      }
      set_value.push(format!("{} = {}", k, v));
    }

    let sql_str = format!(
      "{} {} SET {} WHERE is_deleted = '0' {}",
      UPDATE, self.table_name, set_value.join(", ").replace('\"', "'"), h_wrapper(wrapper),
    );

    println!("update sql:{}", &sql_str);

    let mut pool = self.db.acquire().await?;
    match query(&sql_str).execute(&mut pool).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  /******************************************** UPDATE MAPPER ********************************************/

  /******************************************** SELECT MAPPER ********************************************/

  // 分页条件查询
  pub async fn select_page(self, page: String, wrapper: String) -> Result<PageData<T>, Error> {
    let mut err: Option<Error> = None;
    
    let sql_str = format!(
      "{} {} WHERE is_deleted = '0' {}{}",
      SELECT_FROM,
      self.table_name,
      h_wrapper(wrapper.clone()),
      page,
    );

    println!("select_page sql: {}", &sql_str);
    
    let mut page_data: PageData<T> = PageData {
      total: 0,
      data: vec![],
    };

    let mut pool = self.db.acquire().await?;

    match self.total(wrapper).await {
      Ok(res) => page_data.total = res,
      Err(e) => err = Some(e),
    };
    
    match query_as::<_, T>(&sql_str).fetch_all(&mut pool).await {
      Ok(res) => page_data.data = res,
      Err(e) => err = Some(e),
    };

    if let Some(e) = err { return Err(e); };
    Ok(page_data)
  }

  // 通过条件查询所有数据
  pub async fn select_list(self, wrapper: String) -> Result<Vec<T>, Error> {
    let sql_str = format!(
      "{} {} WHERE is_deleted = '0' {}",
      SELECT_FROM, self.table_name, h_wrapper(wrapper),
    );
    println!("select_list sql: {}", &sql_str);
    let mut pool = self.db.acquire().await?;
    match query_as::<_, T>(&sql_str).fetch_all(&mut pool).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  // 自定义sql 查询列表
  pub async fn select_list_custom(self, sql_str: String) -> Result<Vec<T>, Error> {
    let mut pool = self.db.acquire().await?;
    match query_as::<_, T>(&sql_str).fetch_all(&mut pool).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  // 根据条件 查询一次
  pub async fn select_one(self, wrapper: String) -> Result<Option<T>, Error> {
    let sql_str = format!(
      "{} {} WHERE is_deleted = '0' {} LIMIT 1",
      SELECT_FROM, self.table_name, h_wrapper(wrapper),
    );
    println!("select_list sql: {}", &sql_str);
    let mut pool = self.db.acquire().await?;
    match query_as::<_, T>(&sql_str).fetch_optional(&mut pool).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  // 根据条件查询一条数据
  pub async fn select_one_custom(self, sql_str: String) -> Result<Option<T>, Error> {
    let mut pool = self.db.acquire().await?;
    match query_as::<_, T>(&sql_str).fetch_optional(&mut pool).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  // ids 批量查询
  pub async fn select_batch_ids(self, ids: Vec<u64>) -> Result<Vec<T>, Error> {
    let ids = ids.iter().map(|&n| format!("{}", n)).collect::<Vec<String>>().join(",");
    let sql_str = format!(
      "{} {} WHERE id IN ({})",
      SELECT_FROM, self.table_name, ids
    );
    println!("select_batch_ids sql: {}", &sql_str);
    let mut pool = self.db.acquire().await?;
    match query_as::<_, T>(&sql_str).fetch_all(&mut pool).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  // 通过包装器查询单个数据
  pub async fn select_by_map(self, wrapper: String) -> Result<Option<T>, Error> {

    let sql_str = format!(
      "{} {} WHERE is_deleted = '0' {}",
      SELECT_FROM, self.table_name, h_wrapper(wrapper),
    );
    println!("select_by_map sql: {}", &sql_str);
    let mut pool = self.db.acquire().await?;
    match query_as::<_, T>(&sql_str).fetch_optional(&mut pool).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  // id 查询
  pub async fn select_by_id(self, id: u64) -> Result<Option<T>, Error> {
    let sql_str = format!(
      "{} {} WHERE id = {} AND is_deleted = 0",
      SELECT_FROM, self.table_name, id
    );
    println!("select_by_id sql: {}", &sql_str);
    let mut pool = self.db.acquire().await?;
    match query_as::<_, T>(&sql_str).fetch_optional(&mut pool).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  // 查询总数
  pub async fn total(self, wrapper: String) -> Result<i64, Error> {
    let sql_str = format!("SELECT COUNT(*) FROM {} WHERE is_deleted = 0 {}", self.table_name ,h_wrapper(wrapper));
    println!("total sql: {}", &sql_str);
    let mut pool = self.db.acquire().await?;
    match query_scalar(&sql_str).fetch_one(&mut pool).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  /******************************************** SELECT MAPPER ********************************************/

  /******************************************** DELETE MAPPER ********************************************/


  pub async fn delete_by_id(self, id: u64) -> Result<MySqlQueryResult, Error> {
    let sql_str = format!(
      "{} {} WHERE id = {}",
      DELETE_FROM, self.table_name, id
    );
    println!("delete_by_id sql: {}", &sql_str);
    let mut pool = self.db.acquire().await?;
    match query(&sql_str).execute(&mut pool).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  pub async fn delete_batch_ids(self, ids: Vec<u64>) -> Result<MySqlQueryResult, Error> {
    let ids = ids.iter().map(|&n| format!("{}", n)).collect::<Vec<String>>().join(",");
    let sql_str = format!(
      "{} {} WHERE id IN ({})",
      DELETE_FROM, self.table_name, ids
    );
    println!("delete_batch_ids sql: {}", &sql_str);
    let mut pool = self.db.acquire().await?;
    match query(&sql_str).execute(&mut pool).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  pub async fn delete(self, wrapper: String) -> Result<MySqlQueryResult, Error> {
    let sql_str = format!(
      "{} {} WHERE is_deleted = '0' {}",
      DELETE_FROM, self.table_name, h_wrapper(wrapper),
    );
    println!("delete sql: {}", &sql_str);
    let mut pool = self.db.acquire().await?;
    match query(&sql_str).execute(&mut pool).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  /******************************************** DELETE MAPPER ********************************************/

}

fn h_wrapper(mut wrapper: String) -> String{
  if wrapper.is_empty() == false {
    wrapper = format!("AND {}", wrapper);
  };
  wrapper
}