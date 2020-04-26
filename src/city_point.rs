use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct CityPoint {
    id: usize,
    x: f64,
    y: f64,
}

impl CityPoint {

    pub fn generate(id: usize, x: f64, y: f64) -> Self {
        CityPoint { id, x, y }
    }

    pub fn get_sqr_dist(&self, second: &CityPoint) -> f64 {
        let dist1 = self.x - second.x;
        let dist2 = self.y - second.y;
        dist1 * dist1 + dist2 * dist2
    }
}

pub fn parse_cities(str_in: &String) -> Vec<CityPoint> {
    let mut cities: Vec<CityPoint> = Vec::new();

    for (i, line) in str_in.lines().enumerate() {

        let data: Vec<f64> = line.split(',')
                                .map(|x| f64::from_str(x.trim())
                                .unwrap())
                                .collect();
        
        let c = CityPoint::generate(i, data[1], data[2]);
        cities.push(c);
    }
    cities
}