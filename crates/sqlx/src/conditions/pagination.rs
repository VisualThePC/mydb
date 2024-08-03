#[warn(non_snake_case)]
pub fn page (offset: u32, limit: u32) -> String {
  Page::new(offset, limit)
}
#[derive(Debug, Default)]
pub struct Page {
  pub offset: u32,
  pub limit: u32,
}

impl Page {
  pub fn new(offset: u32, limit: u32) -> String {
    format!(
      "LIMIT {} OFFSET {}",
      limit,
      limit * (offset - 1)
    )
  }
}
