use bevy::prelude::*;

const MAX_POINTS: f32 = 100.0;

#[derive(Debug, Component)]
pub struct City {
    pub name: String,
    pub loc: (f32, f32),
}

#[derive(Debug)]
pub enum Guess {
    Location(f32, f32),
    CityName(String),
}

impl Guess {
    pub fn show_result (&self) {
        match self {
            Guess::Location(x,y) => println!{"Located at ({},{})", x, y},
            Guess::CityName(name) => println!("The City Name is {}", name)
        }
    }
}

pub fn compare_results(guess:&Guess, answer:&City) -> f32 {
    match guess {
        Guess::CityName(name) => {((&name.trim().to_lowercase() == &answer.name.to_lowercase()) as u32 ) as f32  * MAX_POINTS}
        Guess::Location(x,y) => {
            let (cx,cy) = answer.loc;
            let distance = ((cx-x).powi(2) + (cy - y).powi(2)).sqrt();
            MAX_POINTS - distance
        }
    }
}


#[test]
fn test_compare_results() {
    let city_name = String::from("Soignies");
    let city_loc = (10.,18.);
    let city = City {name:city_name.clone(), loc:city_loc};
    let guess1 = Guess::CityName(city_name);
    let guess2 = Guess::Location(4., 10.);
    assert_eq!(compare_results(&guess1, &city), MAX_POINTS);
    assert_eq!(compare_results(&guess2, &city), MAX_POINTS - 10.);
}
