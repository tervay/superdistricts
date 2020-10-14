mod protos;
mod tba;

use protobuf::Message;
use protos::Team::Team;
use std::fs::File;

fn save_proto(team: &Team) -> protobuf::ProtobufResult<()> {
    let mut file = File::create(format!("cache/{}", team.key)).unwrap();
    let mut writer = std::io::BufWriter::new(&mut file);
    return team.write_to_writer(&mut writer);
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let team: protos::Team::Team = tba::get_team(2791).await;
    save_proto(&team);

    println!("{:?}", team);

    Ok(())
}
