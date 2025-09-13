use crate::net;
use http::Uri;
use serde::Deserialize;

const BASE_URL: &str = "https://cflookup.com";

pub type Mod = serde_json::Value;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct GameCategory {
    pub game: String,
    pub game_id: u64,
    pub game_slug: String,
    pub category: String,
    pub category_id: u64,
    pub category_slug: String,

    pub class: bool,
    pub url: Option<Uri>,
}

pub async fn get_project_by_id(project_id: u64) -> anyhow::Result<Option<Mod>> {
    let uri = format!("{BASE_URL}/{project_id}.json");
    net::make_json_request(uri).await
}

pub async fn get_project_by_slug(
    game: &str,
    category: &str,
    slug: &str,
) -> anyhow::Result<Option<Mod>> {
    let uri = format!("{BASE_URL}/{game}/{category}/{slug}.json");
    net::make_json_request(uri).await
}

pub async fn get_file_info(file_id: u64) -> anyhow::Result<Option<Mod>> {
    let uri = format!("{BASE_URL}/file-{file_id}.json");
    net::make_json_request(uri).await
}

pub async fn search_project_by_slug(
    slug: &str,
) -> anyhow::Result<(Vec<(GameCategory, Vec<Mod>)>, usize)> {
    let uri = format!("{BASE_URL}/api/search/slug/{slug}");
    let Some(json) = net::make_json_request(uri).await? else {
        return Ok((vec![], 0));
    };

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SearchResponse {
        data: Vec<SearchResult>,
        total_results: usize,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SearchResult {
        game: SearchResultGame,
        category: SearchResultCategory,
        mods: Vec<Mod>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SearchResultGame {
        id: u64,
        name: String,
        slug: String,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SearchResultCategory {
        id: u64,
        name: String,
        slug: String,
        #[serde(with = "http_serde::option::uri")]
        url: Option<Uri>,
        #[serde(rename = "isClass")]
        class: bool,
    }

    let response = serde_json::from_value::<SearchResponse>(json)?;

    if response.total_results == 0 {
        return Ok((vec![], 0));
    }

    Ok((
        response
            .data
            .iter()
            .map(|result| {
                let category = GameCategory {
                    game: result.game.name.clone(),
                    game_id: result.game.id,
                    game_slug: result.game.slug.clone(),
                    category: result.category.name.clone(),
                    category_id: result.category.id,
                    category_slug: result.category.slug.clone(),

                    class: result.category.class,
                    url: result.category.url.clone(),
                };
                let values = result.mods.clone();

                (category, values)
            })
            .collect(),
        response.total_results,
    ))
}
