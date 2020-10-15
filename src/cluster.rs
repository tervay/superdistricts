use crate::protos::Team::Team;
use crate::Location;
use indicatif::ProgressIterator;

use quaternion::Quaternion;
use rand::Rng;
use vecmath::Vector3;

const DEG2RAD: f64 = (std::f64::consts::PI / 180.0);
const RAD2DEG: f64 = (180.0 / std::f64::consts::PI);

#[derive(Debug, Clone)]
pub struct Point {
    pub team: Team,
    pub score: f64,
}

#[derive(Debug)]
pub struct Cluster {
    pub location: Location,
    pub points: Vec<Point>,
}

#[derive(Debug)]
pub struct Clusterer {
    pub points: Vec<Point>,
    pub clusters: Vec<Cluster>,
}

impl Clusterer {
    pub fn compute_scores(&mut self) {
        let clone = self.points.clone();

        for mut point in self.points.iter_mut().progress() {
            point.score = clone.iter().fold(0.0, |acc, p2| {
                let distance = point
                    .team
                    .get_loc()
                    .distance_to(&p2.team.get_loc())
                    .unwrap()
                    .meters();

                return acc + 1.0 / f64::max(1.0, distance / 1000.0);
            });
        }
        self.points
            .sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    }
}

pub fn make_clusterer(teams: Vec<Team>) -> Clusterer {
    let mut rng = rand::thread_rng();

    let pts: Vec<Point> = teams
        .into_iter()
        .map(|t| Point {
            team: t,
            score: 0.0,
        })
        .collect();
    let pc = pts.clone();
    return Clusterer {
        points: pts,
        clusters: (0..10)
            .map(|_| {
                gen_around_location(
                    pc.get(rng.gen_range(0, pc.len())).unwrap().team.get_loc(),
                    0.1,
                )
            })
            .map(|l| Cluster {
                location: l,
                points: vec![],
            })
            .collect(),
    };
}

fn gen_around_location(loc: Location, r: f64) -> Location {
    let mut rng = rand::thread_rng();
    let lat = loc.latitude();
    let lon = loc.longitude();

    // Transform to cartesian coordinates
    let x = (DEG2RAD * lon).cos();
    let y = (DEG2RAD * lon).sin();
    let z = (DEG2RAD * lat).sin();

    // Generate random unit vector
    let x1 = 2.0 * rng.gen::<f64>() - 1.0;
    let y1 = 2.0 * rng.gen::<f64>() - 1.0;
    let z1 = 2.0 * rng.gen::<f64>() - 1.0;
    let len = (x1 * x1 + y1 * y1 + z1 * z1).sqrt();

    // Generate random angle
    let ang = 0.5 * (r * DEG2RAD) * rng.gen::<f64>();
    let ca = ang.cos();
    let sa = ang.sin() / len;

    // Create Quaternion components
    let vec: Vector3<f64> = [x, y, z]; // Todo handle 0 case
    let q: Quaternion<f64> = (ca, [sa * x1, sa * y1, sa * z1]);
    let vec = quaternion::rotate_vector(q, vec);

    let r_lon = RAD2DEG * vec[1].atan2(vec[0]);
    let r_lat = RAD2DEG * vec[2].asin();
    return Location::new(r_lat, r_lon);
}
