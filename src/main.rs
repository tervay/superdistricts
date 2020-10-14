mod geo;
mod protos;
mod tba;

use protobuf;
use protobuf::Message;
use protos::Team::Team;
use std::fs::File;
use std::fs::OpenOptions;
use std::{thread, time};

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

#[tokio::main]
async fn main() -> Result<(), ()> {
    // let mut team: protos::Team::Team = tba::get_team("frc2791").await;
    // geo::populate_coords(&mut team).await;
    // let result = save_proto(&team);
    // println!("{:#?}", load_proto("frc1"));

    let one_sec = time::Duration::from_secs_f32(1.15);

    let mut teams = tba::get_all_teams().await;
    for team in teams.iter_mut() {
        println!("{}", team.key);
        geo::populate_coords(team).await;
        save_proto(team);
        thread::sleep(one_sec);
    }

    Ok(())
}
