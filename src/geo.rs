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

fn fix_geo(team: &mut Team) {
    team.city = protobuf::Chars::from(match team.city.to_string().as_str() {
        "Falmouth/Gorham" => "Falmouth",
        "Quad Cities" => "Davenport",
        "USAF Academy" => "Colorado Springs",
        "Whitinsville" => "Northbridge",
        "Hatton-Northwood" => "Northwood",
        "Northboro" => "Northborough",
        "O Fallon" => "O'Fallon",
        "currently meeting in Bedford" => "Bedford",
        _ => team.city.chars().as_str(),
    });

    team.state = protobuf::Chars::from(match team.state.to_string().as_str() {
        "Tel-Aviv" => "Tel Aviv District",
        "HaMerkaz" => "Center District",
        _ => team.state.chars().as_str(),
    });
}

pub async fn populate_coords(team: &mut Team) {
    fix_geo(team);
    let json = get_osm_info(team).await;

    team.latitude = json[0]["lat"]
        .as_str()
        .expect(&format!(
            "Failed to find location for {} ({}, {}, {})",
            team.key, team.city, team.state, team.country
        ))
        .parse::<f32>()
        .unwrap();
    team.longitude = json[0]["lon"].as_str().unwrap().parse::<f32>().unwrap();
}
