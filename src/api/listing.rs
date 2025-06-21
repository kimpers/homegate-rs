use reqwest::{Response, Url};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::api::request::get_url;
use crate::api::BACKEND_URL;
use crate::models::address::Address;
use crate::models::listing::Attachment;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListingResponse {
    pub listings: Vec<OuterListing>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListingCard {
    pub size: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListingType {
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OuterListing {
    pub listing: ExtendedListing,
    pub id: String,
    pub listing_card: Option<ListingCard>,
    pub listing_type: Option<ListingType>,
    pub paused: Option<bool>,
    pub remote_viewing: Option<bool>,
    pub lister_branding: Option<ListerBranding>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListerBranding {
    pub logo_url: Option<String>,
    pub legal_name: Option<String>,
    pub address: Option<Address>,
    pub ad_active: Option<bool>,
    pub is_premium_branding: Option<bool>,
    pub profile_page_url_keyword: Option<String>,
    pub is_quality_partner: Option<bool>,
    pub subscription_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Website {
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Lister {
    pub id: Option<String>,
    pub phone: Option<String>,
    pub contacts: Option<Contacts>,
    pub legal_name: Option<String>,
    pub website: Option<Website>,
    pub address: Option<Address>,
    pub logo_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Contacts {
    pub inquiry: Option<Contact>,
    pub viewing: Option<Contact>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub given_name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Legacy {
    pub person_id: Option<i64>,
    pub publishing_group_id: Option<i64>,
    pub ppa_person_id: Option<i64>,
    pub delivery_group_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExternalIds {
    pub internal_reference_id: Option<String>,
    pub ref_object: Option<String>,
    pub display_property_reference_id: Option<String>,
    pub property_reference_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContactForm {
    pub size: Option<String>,
    pub delivery_format: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EwidCandidate {
    pub ewid: Option<String>,
    pub rooms: Option<i32>,
    pub living_space: Option<i32>,
    pub floor: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Bfs {
    pub egrid: Option<String>,
    pub egid: Option<String>,
    pub ewid_candidates: Option<Vec<EwidCandidate>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub source: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublisherOption {
    pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Publisher {
    pub options: Option<Vec<PublisherOption>>,
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Prices {
    pub rent: Option<Price>,
    pub currency: Option<String>,
    pub buy: Option<Price>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub interval: Option<String>,
    pub net: Option<f64>,
    pub gross: Option<f64>,
    pub extra: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ValueAddedServices {
    pub bundle: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedListing {
    pub localization: Localization,
    pub characteristics: Characteristics,
    pub available_from: Option<String>,
    pub description: Option<String>,
    pub lister: Option<Lister>,
    pub legacy: Option<Legacy>,
    pub address: Option<Address>,
    pub external_ids: Option<ExternalIds>,
    pub contact_form: Option<ContactForm>,
    pub bfs: Option<Bfs>,
    pub platforms: Option<Vec<String>>,
    pub offer_type: Option<String>,
    pub meta: Option<Meta>,
    pub publishers: Option<Vec<Publisher>>,
    pub id: Option<String>,
    pub categories: Option<Vec<String>>,
    pub prices: Option<Prices>,
    pub value_added_services: Option<ValueAddedServices>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Characteristics {
    pub is_old_building: Option<bool>,
    pub floor: Option<i32>,
    pub is_quiet: Option<bool>,
    pub has_parking: Option<bool>,
    pub has_washing_machine: Option<bool>,
    pub has_balcony: Option<bool>,
    pub living_space: Option<f64>,
    pub number_of_rooms: Option<f64>,
    pub year_built: Option<i32>,
    pub is_wheelchair_accessible: Option<bool>,
    pub has_elevator: Option<bool>,
    pub is_minergie_certified: Option<bool>,
    pub is_child_friendly: Option<bool>,
    pub has_garage: Option<bool>,
    pub number_of_floors: Option<i32>,
    pub total_floor_space: Option<i32>,
    pub has_cable_tv: Option<bool>,
    pub distance_motorway: Option<i32>,
    pub distance_public_transport: Option<i32>,
    pub is_ground_floor: Option<bool>,
    pub distance_shop: Option<i32>,
    pub year_last_renovated: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Localization {
    pub de: Option<LocalizationEntry>,
    pub en: Option<LocalizationEntry>,
    pub it: Option<LocalizationEntry>,
    pub fr: Option<LocalizationEntry>,
    pub de_de: Option<LocalizationEntry>,
    pub primary: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LocalizationEntry {
    pub attachments: Vec<Attachment>,
    pub text: LocalizationEntryText,
    pub is_machine_translated: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LocalizationEntryText {
    pub title: String,
    pub description: Option<Description>,
    pub situation: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Description {
    String(String),
    Map(HashMap<String, String>),
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
    use crate::api::listing::parse_listing_result;
    use std::fs;

    #[tokio::test]
    pub async fn it_parses_single_listing_from_file() {
        let file = fs::read_to_string("./resources/test/result-1.json").unwrap();
        let listing_response = parse_listing_result(&file);
        assert_eq!(listing_response.listings.len(), 1);
    }

    #[test]
    pub fn parse_json() {
        let file = fs::read_to_string("./resources/test/listing.json").unwrap();
        let listing_response = parse_listing_result(&file);

        assert_eq!(listing_response.listings.len(), 2);
    }

    #[test]
    pub fn parse_full_json() {
        let file = fs::read_to_string("./resources/test/result-1.json").unwrap();
        let listing_response = parse_listing_result(&file);

        assert_eq!(listing_response.listings.len(), 1);
    }

    #[test]
    pub fn parse_full_json_2() {
        let file = fs::read_to_string("./resources/test/result-3.json").unwrap();
        let listing_response = parse_listing_result(&file);

        assert_eq!(listing_response.listings.len(), 4);
    }
}
