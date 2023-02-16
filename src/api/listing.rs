use reqwest::{Response, Url};
use serde::{Deserialize, Serialize};

use crate::api::request::get_url;
use crate::api::BACKEND_URL;
use crate::models::listing::Attachment;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListingResponse {
    pub listings: Vec<OuterListing>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OuterListing {
    pub listing: ExtendedListing,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedListing {
    pub localization: Localization,
    pub characteristics: Characteristics,
    pub available_from: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Characteristics {
    pub is_old_building: Option<bool>,
    pub floor: Option<u32>,
    pub is_quiet: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Localization {
    pub de: Option<LocalizationEntry>,
    pub primary: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LocalizationEntry {
    pub attachments: Vec<Attachment>,
    pub text: LocalizationEntryText,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LocalizationEntryText {
    pub title: String,
    pub description: String,
}

pub async fn get_listings(ids: &[&str]) -> Result<ListingResponse, reqwest::Error> {
    let url: Url = Url::parse(&format!(
        "{}{}?ids={}",
        BACKEND_URL,
        "/listings/listings",
        ids.join(",")
    ))
    .unwrap();

    let resp: Response = get_url(url).await?;
    let resp_text = resp.text().await?;

    let listing_response: ListingResponse = parse_listing_result(&resp_text);
    Ok(listing_response)
}

pub fn parse_listing_result(str: &str) -> ListingResponse {
    serde_json::from_str(str).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::api::listing::{get_listings, parse_listing_result};
    use std::fs;

    #[tokio::test]
    pub async fn it_gets_listing() {
        let listing_response = get_listings(&["3002335392"])
            .await
            .expect("request succeeds");
        assert!(listing_response.listings.len() == 1);
    }

    #[test]
    pub fn parse_json() {
        let file = fs::read_to_string("./resources/test/listing.json").unwrap();
        let listing_response = parse_listing_result(&file);

        assert!(listing_response.listings.len() == 2);
    }
}
