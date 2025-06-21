use crate::api::request;
use crate::models::location::Location;
use std::vec::Vec;

pub async fn get_areas() -> Result<Vec<Location>, reqwest::Error> {
    let r = request::get("/rs/geo-areas?lan=en").await?;
    let res: Vec<Location> = r.json().await?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    pub async fn get_areas() {
        let v = crate::api::geo::get_areas().await;
        if let Ok(locations) = v {
            assert_ne!(0, locations.len());
        }
    }
}
