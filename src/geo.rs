use crate::protos::Team::Team;
use reqwest::header::USER_AGENT;

fn get_osm_info(team: &Team) -> serde_json::Value {
    return reqwest::blocking::Client::new()
        .get("https://nominatim.openstreetmap.org/search")
        .header(USER_AGENT, "FRC SD model")
        .query(&[
            ("city", team.city.to_string()),
            ("state", team.state.to_string()),
            ("country", team.country.to_string()),
            ("format", "json".to_string()),
        ])
        .send()
        .unwrap()
        .json()
        .unwrap();
}

pub fn fix_geo(team: &mut Team) {
    team.city = protobuf::Chars::from(match team.city.to_string().as_str() {
        "Falmouth/Gorham" => "Falmouth",
        "Quad Cities" => "Davenport",
        "USAF Academy" => "Colorado Springs",
        "Whitinsville" => "Northbridge",
        "Hatton-Northwood" => "Northwood",
        "Northboro" => "Northborough",
        "O Fallon" => "O'Fallon",
        "currently meeting in Bedford" => "Bedford", // 1519
        "Kibutz E'in Shemer" => "Ein Shemer",
        "Lutherville Timonium" => "Timonium",
        "Gadera" => "Gedera",
        "Rosh Hayin" => "Rosh HaAyin",
        "Clearview Twp." => "Clearview",
        "Carmel" => {
            if team.key.to_string() == "frc2035" {
                "Carmel-By-The-Sea"
            } else {
                // 5943
                "Carmel"
            }
        }
        "Southbury / Middlebury" => "Middlebury",
        "APO" => "Wiesbaden",                             // 3011
        "Kefar Blum" => "Kfar Blum",                      // 3034
        "Coeur D Alene" => "Coeur d'Alene",               // 3145
        "Jamaica Estates" => "Queens",                    // 3204
        "Atizapan de Zaragoza" => "Ciudad López Mateos", // 3274
        "Indian River" => "Miami Beach",                  // 3537
        "Lackland Air Force Base" => "San Antonio",       // 3545
        "Storrs Mansfield" => "Storrs",                   // 3555
        "Brownstown" => "Rockwood",                       // 3604
        "Blue Earth Area" => "Blue Earth",                // 4260
        "Petach Tikvah" => "Petah Tikva",                 // 4320
        "Cuautilán Izcalli" => "Cuautitlán Izcalli",    // 4371
        "Laguna del Rey Coahuila" => "Laguna del Rey",    // 4401
        "Mayes County " => "Locust Grove",                // 4523
        "Research Triangle Park" => "Research Triangle",  // 4561
        "bet-hasmonai" => "Azarya",                       // 4661
        "Belle River" => "Saint Joachim",                 // 4688
        // 5038, RTL machine broke
        "Megiddo Regional Council " => "מגידו",
        "Standoff" => "Stand Off",      // 5118
        "Dabburiya" => "Nazareth",      // 5715
        "Lethbridge" => "Coalhurst",    // 5725
        "Petah Tiqua" => "Petah Tikva", // 5928
        "Santa Catarina / Tuxtla Gutierrez, Chiapas" => "Santa Catarina", // 6017
        "Chiayi" => "Chiayi City",      // 6083
        "Eden Valley-Watkins" => "Eden Valley", // 6175
        "Middleton" => {
            if team.key.to_string() == "frc6638" {
                "Fulton Township"
            } else {
                "Middleton"
            }
        }
        "rishon le tzion" => "Rishon LeZion",          // 6741
        "Döşemealtı" => "Antalya",                  // 6874
        "East Brunswick" => "New Brunswick",           // 6897
        "İstanbul/Beşiktaş" => "İstanbul",         // 7035
        "Kfar hanoar Neurim " => "Ne'urim",            // 7039
        "Wuri Dist." => "Wuri District",               // 7130,
        "taybe" => "Tayibe",                           // 7177
        "Atatürk Mah." => "Atatürk",                 // 7458
        "Jaffa of Nazareth" => "Nazareth",             // 7554
        "Hoolehua" => "Moloka'i",                      // 7724
        "Beşiktaş" => "İstanbul",                   // 7839
        "BEYLİKDÜZÜ / İstanbul" => "Beylikdüzü", // 8147
        "Kangshan" => "Gangshan District",             // 8169
        "Tainan City" => "Tainan",                     // 8178
        _ => team.city.chars().as_str(),
    });

    team.state = protobuf::Chars::from(match team.state.to_string().as_str() {
        "Tel-Aviv" => {
            if team.key.to_string() == "frc6741" {
                ""
            } else {
                "Tel Aviv District"
            }
        }
        "HaMerkaz" => "Center District",
        "HaDarom" => "",
        "HaZafon" => "",
        "Distrito Federal" => "",
        "Yerushalayim" => "",
        "Kaohsiung" => "",                   // 7526
        "Taipei Special Municipality" => "", // 7589
        "Tainan Municipality" => "",         // 8178
        _ => team.state.chars().as_str(),
    });

    team.country = protobuf::Chars::from(match team.country.to_string().as_str() {
        "Chinese Taipei" => "Taiwan", // 4253
        _ => team.country.chars().as_str(),
    });
}

pub fn populate_coords(team: &mut Team) {
    fix_geo(team);
    let json = get_osm_info(team);

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
