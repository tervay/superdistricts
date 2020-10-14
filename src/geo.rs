use crate::protos::Team::Team;
use reqwest::header::USER_AGENT;

async fn get_osm_info(team: &Team) -> serde_json::Value {
    return reqwest::Client::new()
        .get("https://nominatim.openstreetmap.org/search")
        .header(USER_AGENT, "FRC SD model")
        .query(&[
            ("city", team.city.to_string()),
            ("state", team.state.to_string()),
            ("country", team.country.to_string()),
            ("format", "json".to_string()),
        ])
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
}

pub async fn populate_coords(team: &mut Team) {
    let json = get_osm_info(team).await;

    team.latitude = json[0]["lat"].as_str().unwrap().parse::<f32>().unwrap();
    team.longitude = json[0]["lon"].as_str().unwrap().parse::<f32>().unwrap();
}
