struct Sheep {}

struct Dog {}

trait Animal {
    fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        return "Mehehehhe";
    }
}

impl Animal for Dog {
    fn noise(&self) -> &'static str {
        return "Rauf rauf";
    }
}


fn random_animal(number: f64) -> Box<dyn Animal> {
    if number < 0.5 {
        return Box::new(Sheep {});
    } else {
        return Box::new(Dog {});
    }
}

fn main() {
    let mut random_number = 0.2;
    let mut animal = random_animal(random_number);
    println!("The Animal says: {}", animal.noise());
    random_number=0.51;
    animal = random_animal(random_number);
    println!("The Animal says: {}", animal.noise());

}