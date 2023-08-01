use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Pagination {
    pub page: u64,
    pub per_page: u64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 15,
        }
    }
}

impl Pagination {
    #[allow(dead_code)]
    pub fn new(page: u64, per_page: u64) -> Self {
        Self { page, per_page }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct PageInfo {
    #[serde(default)]
    pub next: Option<u64>,
    #[serde(default)]
    pub pages: Vec<Option<u64>>,
    #[serde(default)]
    pub prev: Option<u64>,
}

impl PageInfo {
    pub fn new(
        page: u64,
        max: u64,
        left_edge: u64,
        left_current: u64,
        right_current: u64,
        right_edge: u64,
    ) -> Self {
        let mut pages = Vec::with_capacity(10);
        let mut last: u64 = 1;
        let page = std::cmp::max(1, page);
        let prev = if page > 1 { Some(page - 1) } else { None };
        let next = if page < max { Some(page + 1) } else { None };
        let left_limit = page.saturating_sub(left_current.saturating_sub(1));
        let right_min = max.saturating_sub(right_edge);
        let right_limit = page.saturating_add(right_current);

        pages.push(Some(1));

        for num in 2..max {
            if num <= left_edge || (num > left_limit && num < right_limit) || num > right_min {
                if last.saturating_add(1) != num {
                    pages.push(None);
                }

                pages.push(Some(num));
                last = num;
            }
        }

        if max != 1 {
            pages.push(Some(max));
        }

        PageInfo { next, pages, prev }
    }
}

pub fn check_page(current_page: &u64, num: &u64) -> bool {
    *current_page == *num
}
