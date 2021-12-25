use std::io;
use std::collections::HashMap;
pub struct Age {
    pub age: u32
}
impl Age {
    pub fn new() -> Age {
        let age: u32;
        println!("How old are you? Input a number of years");
        loop {
            let mut number=String::new();
            io::stdin().read_line(&mut number).
                expect("Failed to read");
            let x: u32 = match number.trim().parse() {
                Ok(y)=> y,
                Err(_)=> {  
                    println!("A number!");
                    continue;
                },
            };
            age=x;
            break;
        };
        Age{age:age}
    }
    pub fn in_seconds(&self)->u32 {
        let age_in_seconds = &self.age*31557600;
        println!("Your age in seconds is {}", age_in_seconds);
        age_in_seconds
    }
    pub fn planet_age(&self, p:&Planet) -> f64{
        let age: f64 = (self.age).into();
        let planet_age:f64 = p.year_duration*age;       println!("Your age on planet {} is {}", p.name, planet_age);
        planet_age
    }
}
pub struct Planet {
    pub name: String,
    pub year_duration: f64,
}
impl Planet {
    pub fn new() -> Planet {
        let names = vec!["Mercury","Venus","Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune"];
        let year_durations: Vec<f64> = vec![0.2408467,0.61519726,1.0,1.8808158,11.862615,29.447498,84.016846,164.79132];
        let mut planets: HashMap<_, _> =
        names.into_iter().zip(year_durations.into_iter()).collect();
        let mut name: String = "".to_string();
        let mut year_duration: f64 = 0.0;
        loop {
            println!("Choose your planet: Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune");
            let mut your_planet=String::new();
            io::stdin().read_line(&mut your_planet).
                expect("Failed to read");
            let your_planet=your_planet.trim().to_string();
            println!("{}", your_planet);
            for (key, value) in &planets {
                println!("{}",key);
                if your_planet==key.to_string() {
                    name=key.to_string();
                    year_duration=*value;
                    break;
                };
            };
            println!("There is no such planet");
        };
        Planet{name:name, year_duration:year_duration}
    }
}
