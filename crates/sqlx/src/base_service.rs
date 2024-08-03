use crate::BaseEntity;
use crate::BaseMapper;
use serde::Serialize;
use sqlx::mysql::MySqlRow;
use sqlx::{ FromRow, Error };
use sqlx_core::mysql::MySqlQueryResult;
use core::fmt::Debug;
use sqlx::MySqlPool;
use crate::PageData;

pub fn service<T> (db: MySqlPool) -> BaseService<T> 
where 
T: for<'a> FromRow<'a, MySqlRow> + Send + Unpin + Debug + Serialize + BaseEntity
{
  BaseService {
    mapper: BaseMapper::<T>::new(db)
  }
}

#[derive(Debug, Clone)]
pub struct BaseService<T: BaseEntity> {
  pub mapper: BaseMapper<T>,
}

impl<T: for<'a> FromRow<'a, MySqlRow> + Send + Unpin + Debug + Serialize + BaseEntity>
BaseService<T>
{

  pub async fn get_by_id(self, id: u64) -> Result<Option<T>, Error>
  {
    match self.mapper.select_by_id(id).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  pub async fn get_one(self, wrapper: String) -> Result<Option<T>, Error> {
    match self.mapper.select_one(wrapper).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  pub async fn all(self) -> Result<Vec<T>, Error> {
    match self.mapper.select_list(format!("")).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  pub async fn list(self, wrapper: String) -> Result<Vec<T>, Error> {
    match self.mapper.select_list(wrapper).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  pub async fn list_by_ids(self, ids: Vec<u64>) -> Result<Vec<T>, Error> {
    match self.mapper.select_batch_ids(ids).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  pub async fn list_by_map(self, wrapper: String) -> Result<Option<T>, Error> {
    match self.mapper.select_by_map(wrapper).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  pub async fn page(self, page: String, wrapper: String) -> Result<PageData<T>, Error> {
    match self.mapper.select_page(page, wrapper).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  pub async fn count(self, wrapper: String) -> Result<i64, Error> {
    match self.mapper.total(wrapper).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  pub async fn save(self, entity: T) -> Result<MySqlQueryResult, Error> {
    match self.mapper.insert(entity).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  pub async fn save_batch(self, entity_vec: Vec<T>) -> Result<MySqlQueryResult, Error> {
    match self.mapper.insert_batch_some_column(entity_vec).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  pub async fn update(self, entity: T, wrapper: String) -> Result<MySqlQueryResult, Error> {
    match self.mapper.update(entity, wrapper).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  pub async fn update_by_id(self, entity: T) -> Result<MySqlQueryResult, Error> {
    match self.mapper.update_by_id(entity).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  pub async fn remove(self, wrapper: String) -> Result<MySqlQueryResult, Error> {
    match self.mapper.delete(wrapper).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  pub async fn remove_by_id(self, id: u64) -> Result<MySqlQueryResult, Error> {
    match self.mapper.delete_by_id(id).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

  pub async fn remove_by_ids(self, ids: Vec<u64>) -> Result<MySqlQueryResult, Error> {
    match self.mapper.delete_batch_ids(ids).await {
      Ok(res) => Ok(res),
      Err(err) => Err(err),
    }
  }

}
