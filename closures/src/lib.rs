use std::thread;
use std::time::Duration;
pub fn generate_workout(intensity:u32, random_number:u32) {
    let mut simulated_expensive_result = Cacher::new(|num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
});
    if intensity<25{
        println!("Do {} pushups", simulated_expensive_result.value(intensity));
        println!("Next, do {} standups", simulated_expensive_result.value(intensity));
    } else {
        if random_number==3 {
            println!("Take a break");
        } else {
            println!("Run for {} minutes", simulated_expensive_result.value(intensity));
        }
    }
}
pub struct Cacher<T>
where T:Fn(u32)->u32, {
        calculation: T,
        value: Option<u32>,
    }
impl<T> Cacher<T>
where T:Fn(u32)->u32, {
    fn new(calculation: T)-> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg:u32) -> u32 {
        match self.value {
            Some(v)=>v,
            None=> {
                let v=(self.calculation)(arg);
                self.value=Some(v);
                v
            }
        }
    }
}
