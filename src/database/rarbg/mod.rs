use std::cmp::max;

use serde::Serialize;
use sqlx::SqlitePool;

pub mod category;

const MAX_RESULTS: usize = 100;

#[derive(Debug, Clone)]
pub struct Database {
    db: SqlitePool,
}

#[derive(Debug, Serialize)]
pub struct RarbgTorrent {
    pub title: String,
    pub category: String,
    pub download: String,
    pub seeders: Option<i64>,
    pub leechers: Option<i64>,
    pub publish_date: String,
    pub episode_info: RarbgTorrentInfo,
    pub ranked: Option<i64>,
    pub info_page: String,
}

#[derive(Debug, Serialize)]
pub struct RarbgTorrentInfo {
    pub imdb: String,
    pub tvrage: Option<i64>,
    pub tvdb: Option<i64>,
}

impl From<Item> for RarbgTorrent {
    fn from(value: Item) -> Self {
        RarbgTorrent {
            title: value.title,
            category: value.cat,
            download: value.hash,
            seeders: None,
            leechers: None,
            publish_date: value.dt,
            episode_info: RarbgTorrentInfo {
                imdb: value.imdb.unwrap_or_default(),
                tvrage: None,
                tvdb: None,
            },
            ranked: None,
            info_page: "".to_string(),
        }
    }
}

#[derive(sqlx::FromRow, Serialize)]
// NOTE: allow(dead_code) rationale: These fields are necessary for SQLX,
// but may not be used in the code
#[allow(dead_code)]
pub struct Item {
    id: i64,
    title: String,
    hash: String,
    dt: String,
    cat: String,
    imdb: Option<String>,
    size: Option<i64>,
    ext_id: Option<String>,
}

#[derive(Debug, Default)]
pub struct Pagination {
    pub offset: usize,
    pub amount: Option<usize>,
}

impl Pagination {
    fn amount(&self) -> usize {
        match self.amount {
            Some(a) => max(a, MAX_RESULTS),
            None => 10,
        }
    }
}

impl Database {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }

    pub async fn search_query(
        &self,
        q: &str,
        categories: Option<&[category::Category]>,
        imdb_id: Option<String>,
        pagination: Option<Pagination>,
    ) -> anyhow::Result<Vec<Item>> {
        let title_query = format!("%{q}%");
        let pagination = pagination.unwrap_or_default();
        let amount = pagination.amount();
        let mut category_params_start: usize = 4;

        let imdb_query = if imdb_id.is_some() {
            category_params_start += 1;
            "AND imdb = ?4"
        } else {
            ""
        };

        let category_names =
            categories.map(|categories| categories.iter().map(|c| c.name()).collect::<Vec<_>>());
        let category_query = if let Some(categories) = categories {
            let category_params = categories
                .iter()
                .enumerate()
                .map(|(i, _)| format!("?{}", i + category_params_start))
                .collect::<Vec<_>>()
                .join(",");
            format!("AND cat IN ({})", category_params)
        } else {
            String::default()
        };

        let query_str = format!(
            r#"
            SELECT * FROM items
            WHERE title like ?1
              {imdb_query}
              {category_query}
              AND id NOT IN (
                SELECT id FROM items
                WHERE title LIKE ?1
                  {imdb_query}
                  {category_query}
                ORDER BY id DESC LIMIT ?3
              )
            ORDER BY id DESC LIMIT ?2"#
        );

        let mut db_query = sqlx::query_as(&query_str);
        db_query = db_query.bind(title_query); // ?1
        db_query = db_query.bind(amount as u32); // ?2
        db_query = db_query.bind(pagination.offset as u32); // ?3
        if let Some(imdb_id) = imdb_id {
            db_query = db_query.bind(imdb_id); // ?4
        }
        if let Some(categories) = category_names {
            for category in categories {
                db_query = db_query.bind(category);
            }
        }

        Ok(db_query.fetch_all(&self.db).await?)
    }
}
