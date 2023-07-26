use axum::body::Body;
use axum::extract::{Query, State};
use axum::http::Request;
use axum::routing::get;
use axum::Router;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::formats::SemicolonSeparator;
use serde_with::{serde_as, StringWithSeparator};
use sqlx::SqlitePool;
use tracing::{debug, trace};

use crate::database::rarbg::category::Category;
use crate::database::rarbg::{Database, Pagination, RarbgTorrent};
use crate::tvdb::SonarrClient;

use super::AppError;

#[derive(Debug, Clone)]
struct AppState {
    db: Database,
    sonarr_client: SonarrClient,
}

pub async fn router() -> anyhow::Result<Router> {
    let db = SqlitePool::connect("sqlite:rarbg_db.sqlite").await?;
    let sonarr_client = SonarrClient::new(
        "http://localhost:8989".to_string(),
        "77d2529ab0a8497eb144f2145d3d0bf5".to_string(),
    )?;
    Ok(Router::new()
        .route("/pubapi_v2.php", get(api))
        .with_state(AppState {
            db: Database::new(db),
            sonarr_client,
        }))
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
struct Params {
    search_string: Option<String>,
    #[serde_as(as = "Option<StringWithSeparator::<SemicolonSeparator, Category>>")]
    category: Option<Vec<Category>>,
    get_token: Option<String>,
    search_tvdb: Option<String>,
    limit: Option<usize>,
}

async fn api(
    Query(params): Query<Params>,
    State(state): State<AppState>,
    req: Request<Body>,
) -> Result<String, AppError> {
    debug!("params: {:?}", params);
    debug!("uri: {:?}", req.uri());

    if params.get_token.is_some() {
        return Ok(json!({
            "token": "bogus"
        })
        .to_string());
    }

    let pagination = Pagination {
        amount: params.limit,
        ..Default::default()
    };

    let imdb_id = if let Some(tvdb_id) = params.search_tvdb {
        let show = state.sonarr_client.get_series_metadata(&tvdb_id).await?;
        show.into_iter().find_map(|s| s.imdb_id)
    } else {
        None
    };

    let r: Vec<RarbgTorrent> = state
        .db
        .search_query(
            &params.search_string.unwrap_or_default(),
            params.category.as_ref().map(|r| r.as_ref()),
            imdb_id,
            Some(pagination),
        )
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

    let res = json!({
        "torrent_results": r,
        "rate_limit": null,
        "error_code": null,
        "error": "",
    })
    .to_string();

    trace!("Responding with: {}", res);

    Ok(res)
}
