### mydb
mydb is a plugin that automatically generates entities and simplifies database operations

##### app Cargo.toml

```rust
sqlx = { version = "0.5.13" , features = ["mysql", "chrono", "uuid", "macros", "runtime-tokio-native-tls"] }
sqlx-core = "0.5.13"
mydb_macros = "0.0.2"
mydb-sqlx = "0.0.2"
```

##### init database

```rust
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub fn init_pool() -> Result<MySqlPool, sqlx::Error> {
  match MySqlPoolOptions::new()
    .min_connections(5)
    .max_connections(10).connect_lazy("mysql://root:root@localhost:3306/oa") {
    Ok(pool) => {
      Ok(pool)
    }
    Err(err) => {
      Err(err)
    }
  }
}

```


##### create file entity/sys_user.rs

```rust
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use sqlx::FromRow;
use mydb_macros::TableData;

#[derive(Debug, Clone, Default, FromRow, Deserialize, Serialize)]
#[derive(TableData)]
#[mydb(table_name = "sys_user")]
pub struct Model {
  pub id: Option<u64>,
  pub username: String,
  pub password: String,
  pub name: Option<String>,
  pub phone: Option<String>,
  pub create_time: Option<DateTime<Utc>>,

  #[table_field(exist=false)]
  pub role_ids: Vec<u64>,
}

```

##### create file entity/sys_user.rs

```rust
use crate::entity::sys_user::Entity;
use mydb_sqlx::{ BaseMapper, mapper };
pub fn mapper() -> BaseMapper<Entity>{
  mapper::<Entity>(init_pool.unwrap())
}

```

##### select_by_id query data

```rust
match mapper().select_by_id(id).await {
  Ok(Some(sys_user)) => {
    println!("query sys_user data :{:#?}", sys_user);
  },
  Ok(None) => println!("no query user"),
  Err(err) => println!(err.to_string()),
}

```

##### create entity/sys_user_qo.rs

```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use mydb_macros::Getters;
use mydb_sqlx::{ query_wrapper, page };

#[derive(Getters)]
#[derive(Debug, Clone, Default, FromRow, Deserialize, Serialize)]
pub struct SysUserQo {
  username: Option<String>,
  start_time: Option<String>,
  end_time: Option<String>,
  page: u32,
  limit: u32,
}

impl SysUserQo {
  pub fn page(&self) -> String {
    page(self.get_page(), self.get_limit())
  }
  pub fn wrapper(&self) -> String {
    query_wrapper()
      .like("username", self.get_username())
      .range("create_time", self.get_start_time(), self.get_end_time())
      .build()
  }
}

```

##### wrapper and page query data

```rust
let body = SysUserQo {
  username: Some("admin"),
  start_time: "2024-01-01 00:00:00"
  end_time: "2024-02-01 23:59:59"
}

match mapper().select_page(body.page(), body.wrapper()).await {
  Ok(sys_user_page) => {
    println!("query sys_user_page data :{:#?}", sys_user_page);
  },
  Err(err) => println!(err.to_string()),
}

```

...Documentation is still being written...
