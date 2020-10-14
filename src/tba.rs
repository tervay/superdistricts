static TBA_KEY: &str = "FfBdTrj0DX7qOqbIaLYYQ0i5HemtJYC2S6OlYl12ODrFdjdpMwG176m0zcL2Jtwn";
static TBA_URL: &str = "https://www.thebluealliance.com/api/v3/";

fn form_url(path: &str) -> String {
    let mut string = String::from(TBA_URL);
    string.push_str(path);
    return string;
}

async fn get(path: &str) -> serde_json::Value {
    return reqwest::Client::new()
        .get(form_url(path).as_str())
        .header("X-TBA-Auth-Key", TBA_KEY)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
}

pub async fn get_team(team_key: &str) -> crate::protos::Team::Team {
    let json = get(&format!("team/{}", team_key)).await;
    let mut team = crate::protos::Team::Team::new();
    team.key = protobuf::Chars::from(json["key"].as_str().unwrap());
    team.name = protobuf::Chars::from(json["name"].as_str().unwrap());
    team.city = protobuf::Chars::from(json["city"].as_str().unwrap());
    team.state = protobuf::Chars::from(json["state_prov"].as_str().unwrap());
    team.country = protobuf::Chars::from(json["country"].as_str().unwrap());
    team.postal_code = protobuf::Chars::from(json["postal_code"].as_str().unwrap());
    return team;
}
