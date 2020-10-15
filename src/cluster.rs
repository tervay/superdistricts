use crate::protos::Team::Team;
use geoutils::Location;
use indicatif::ProgressIterator;

#[derive(Debug, Clone)]
pub struct Point {
    pub team: Team,
    pub score: f64,
}

#[derive(Debug)]
pub struct Clusterer {
    pub points: Vec<Point>,
}

impl Clusterer {
    pub fn compute_scores(&mut self) {
        let clone = self.points.clone();
        for mut point in self.points.iter_mut().progress() {
            let p1 = Location::new(point.team.latitude, point.team.longitude);

            for point2 in clone.clone() {
                let p2 = Location::new(point2.team.latitude, point2.team.longitude);
                let distance = p1.distance_to(&p2).unwrap().meters() / 1000.0;

                point.score += 1.0 / f64::max(1.0, distance);
            }
        }

        self.points
            .sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    }
}

pub fn make_clusterer(teams: Vec<Team>) -> Clusterer {
    return Clusterer {
        points: teams
            .into_iter()
            .map(|t| Point {
                team: t,
                score: 0.0,
            })
            .collect(),
    };
}
