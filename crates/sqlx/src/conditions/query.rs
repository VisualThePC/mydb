
#[warn(non_snake_case)]
pub fn query_wrapper () -> QueryWrapper {
  QueryWrapper::new()
}

#[derive(Debug, Clone, Default)]
pub struct QueryWrapper {
  wrapper: Vec<String>,
}

impl QueryWrapper {
  pub fn new() -> Self {
    Self {
      wrapper: vec![]
    }
  }

  pub fn like<T>(&mut self, key: &'static str, value: Option<T>) -> &mut Self
    where
      T: std::fmt::Display,
  {
    if let Some(v) = value {
      self.wrapper.push(format!("{} LIKE '%{}%'", key, v));
    }
    self
  }

  pub fn left_like<T>(&mut self, key: &'static str, value: Option<T>) -> &mut Self
    where
      T: std::fmt::Display,
  {
    if let Some(v) = value {
      self.wrapper.push(format!("{} LIKE '%{}'", key, v));
    }
    self
  }
  
  pub fn right_like<T>(&mut self, key: &'static str, value: Option<T>) -> &mut Self
    where
      T: std::fmt::Display,
  {
    if let Some(v) = value {
      self.wrapper.push(format!("{} LIKE '{}%'", key, v));
    }
    self
  }

  pub fn eq<T>(&mut self, key: &'static str, value: Option<T>) -> &mut Self
    where
      T: std::fmt::Display,
  {
    if let Some(v) = value {
      self.wrapper.push(format!("{} = '{}'", key, v));
    }
    self
  }

  pub fn ne<T>(&mut self, key: &'static str, value: Option<T>) -> &mut Self
    where
      T: std::fmt::Display,
  {
    if let Some(v) = value {
      self.wrapper.push(format!("{} != '{}'", key, v));
    }
    self
  }

  pub fn gt<T>(&mut self, key: &'static str, value: Option<T>) -> &mut Self
    where
      T: std::fmt::Display,
  {
    if let Some(v) = value {
      self.wrapper.push(format!("{} > '{}'", key, v));
    }
    self
  }

  pub fn lt<T>(&mut self, key: &'static str, value: Option<T>) -> &mut Self
    where
      T: std::fmt::Display,
  {
    if let Some(v) = value {
      self.wrapper.push(format!("{} < '{}'", key, v));
    }
    self
  }

  pub fn ge<T>(&mut self, key: &'static str, value: Option<T>) -> &mut Self
    where
      T: std::fmt::Display,
  {
    if let Some(v) = value {
      self.wrapper.push(format!("{} >= '{}'", key, v));
    }
    self
  }

  pub fn le<T>(&mut self, key: &'static str, value: Option<T>) -> &mut Self
    where
      T: std::fmt::Display,
  {
    if let Some(v) = value {
      self.wrapper.push(format!("{} <= '{}'", key, v));
    }
    self
  }

  pub fn range<T>(
    &mut self,
    key: &'static str,
    s: Option<T>,
    e: Option<T>,
  ) -> &mut Self
    where
      T: std::fmt::Display,
  {
    if let Some(v) = s {
      self.wrapper.push(format!("{} >= '{}'", key, v));
    }
    if let Some(v) = e {
      self.wrapper.push(format!("{} <= '{}'", key, v));
    }
    self
  }

  pub fn build(&mut self) -> String {
    self.wrapper.join(" AND ")
  }
}
