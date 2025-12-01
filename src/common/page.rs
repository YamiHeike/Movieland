use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Page {
    page_number: Option<u64>,
    page_size: Option<u64>,
}