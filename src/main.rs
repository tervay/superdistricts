mod cluster;
mod geo;
mod protos;
mod tba;

use geoutils::Location;
use protobuf;
use protobuf::Message;
use protos::Team::Team;
use std::env;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::{thread, time};
use webbrowser;

impl Team {
    pub fn get_loc(&self) -> Location {
        return Location::new(self.latitude, self.longitude);
    }
}

fn get_file(fp: String) -> File {
    return OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(fp)
        .unwrap();
}

fn save_proto(team: &Team) -> protobuf::ProtobufResult<()> {
    let mut file = get_file(format!("cache/{}", team.key));
    let mut writer = std::io::BufWriter::new(&mut file);
    return team.write_to_writer(&mut writer);
}

fn load_proto(team_key: &str) -> Team {
    let mut file = get_file(format!("cache/{}", team_key));
    let mut reader = std::io::BufReader::new(&mut file);
    let res = protobuf::parse_from_reader::<Team>(&mut reader);

    return res.unwrap();
}

fn load_all_protos() -> Vec<Team> {
    let paths = fs::read_dir("cache/").unwrap();

    return paths
        .into_iter()
        .map(|p| load_proto(p.unwrap().file_name().to_str().unwrap()))
        .collect();
}

fn file_exists(fp: String) -> bool {
    return Path::new(fp.as_str()).exists();
}

fn download() {
    let one_sec = time::Duration::from_secs_f32(1.15);

    let mut teams = tba::get_all_teams();
    for team in teams.iter_mut() {
        if file_exists(format!("cache/{}", team.key)) {
            println!("Skipping {}", team.key);
            continue;
        }

        println!("{}", team.key);
        geo::populate_coords(team);
        save_proto(team).expect("asdf");
        thread::sleep(one_sec);
    }
}

fn debug() {
    let mut team: protos::Team::Team = tba::get_team("frc2791");
    geo::populate_coords(&mut team);
    let result = save_proto(&team);
    result.expect("msg");
    println!("{:#?}", load_proto("frc1"));
}

fn search(team_key: &str) {
    let mut team = tba::get_team(team_key);
    geo::fix_geo(&mut team);
    let client = reqwest::blocking::Client::new()
        .get("https://nominatim.openstreetmap.org/search")
        .header(reqwest::header::USER_AGENT, "FRC SD model")
        .query(&[
            ("city", team.city.to_string()),
            ("state", team.state.to_string()),
            ("country", team.country.to_string()),
            ("format", "json".to_string()),
        ])
        .build()
        .unwrap();

    let url = client.url();
    webbrowser::open(url.as_str()).expect("Unable to open url");
}

fn cluster() {
    let mut c = cluster::make_clusterer(load_all_protos());
    c.compute_scores();
    c.iterate();

    println!("{:#?}", c.clusters);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).expect("wrong usage").as_str() {
        "download" => download(),
        "debug" => debug(),
        "search" => search(args.get(2).expect("missing 2nd arg")),
        "cluster" => cluster(),
        _ => panic!("Wrong usage"),
    };
}
