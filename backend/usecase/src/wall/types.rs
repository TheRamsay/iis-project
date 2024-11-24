use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SortBy {
    #[default]
    Newest,
    Oldest,
    MostLiked,
}
