use reqwest;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use serde_json::json;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultsRoot {
    pub version: i64,
    pub limit: i64,
    pub valid: bool,
    pub results: Vec<SearchResults>,
    #[serde(rename = "num_pages")]
    pub num_pages: i64,
    pub page: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResults {
    pub pkgname: String,
    pub pkgbase: String,
    pub repo: String,
    pub arch: String,
    pub pkgver: String,
    pub pkgrel: String,
    pub epoch: i64,
    pub pkgdesc: String,
    pub url: String,
    pub filename: String,
    #[serde(rename = "compressed_size")]
    pub compressed_size: i64,
    #[serde(rename = "installed_size")]
    pub installed_size: i64,
    #[serde(rename = "build_date")]
    pub build_date: String,
    #[serde(rename = "last_update")]
    pub last_update: String,
    #[serde(rename = "flag_date")]
    pub flag_date: Value,
    pub maintainers: Vec<String>,
    pub packager: String,
    pub groups: Vec<Value>,
    pub licenses: Vec<String>,
    pub conflicts: Vec<String>,
    pub provides: Vec<String>,
    pub replaces: Vec<Value>,
    pub depends: Vec<String>,
    pub optdepends: Vec<String>,
    pub makedepends: Vec<String>,
    pub checkdepends: Vec<String>,
}

/// Preforms a loose search (name, description mentions) for a package
pub async fn search_packages_loose(pkg_name: String) -> Vec<SearchResults>
{
    let search_results : SearchResultsRoot = reqwest::Client::new()
        .get(format!("https://archlinux.org/packages/search/json/?q={}", pkg_name)).send()
        .await.unwrap()
        .json().await.unwrap();

    let search_results_vec : Vec<SearchResults> = search_results.results;
    
    return search_results_vec;
}

// Preforms an exact search for a function
pub async fn search_packages_exact(pkg_name: String)
{
    // TODO:
}