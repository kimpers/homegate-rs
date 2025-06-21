use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Paginated<T> {
    pub from: Option<u32>,
    pub size: Option<u32>,
    pub total: Option<u32>,
    pub results: Option<Vec<T>>,
}

pub fn parse_search_result<T: DeserializeOwned>(str: &str) -> Paginated<T> {
    serde_json::from_str(str).unwrap()
}

#[cfg(test)]
mod test {
    use crate::models::paginated::parse_search_result;
    use crate::models::realestate::RealEstate;
    use std::fs;

    #[test]
    pub fn parse_result_2() {
        let file = fs::read_to_string("./resources/test/result-2.json").unwrap();
        let paginated_result = parse_search_result::<RealEstate>(&file);

        assert_eq!(paginated_result.results.unwrap().len(), 20)
    }
}
