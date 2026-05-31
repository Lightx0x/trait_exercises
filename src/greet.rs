// **Exercise 2 — `Greet` trait**
// 
// Define a `Greet` trait with method `greet(&self) -> String`. Implement it for three types:
// 
// - `Person { name: String }`
// - `Dog { name: String }`
// - `Robot { id: u32 }`
// 
// Each implementation should return a different greeting message.
// 
// Write a generic function `announce<T: Greet>(item: &T)` that prints the greeting.
// 
// Write a test that creates one of each type and passes them to `announce`.
// 
// **Bonus:** add a second trait `Farewell` with method `farewell(&self) -> String`. Implement it for `Person` and `Dog` only. Then write a function bounded by both traits — `<T: Greet + Farewell>` — that calls both methods.

pub trait Greet {
    fn greet(&self) -> String;
}

pub trait Farewell {
    fn farewell(&self) -> String;
}

pub struct Person {
    pub name: String,
}

pub struct Dog {
    pub name: String,
}

pub struct Robot {
    pub id: u32,
}

impl Person {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Dog {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Robot {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

impl Greet for Person {
    fn greet(&self) -> String {
        format!("Welcome to Rustinun: {}", self.name)
    }
}

impl Greet for Dog {
    fn greet(&self) -> String {
        format!("Welcome to your Pet Dog: {}", self.name)
    }
}

impl Greet for Robot {
    fn greet(&self) -> String {
        format!("Welcome to your bot: {}", self.id)
    }
}

pub fn announce_greeting<T: Greet>(item: &T) {
    println!("{}", item.greet());
}

impl Farewell for Person {
    fn farewell(&self) -> String {
        format!("See you later {}", self.name)
    }
}

impl Farewell for Dog {
    fn farewell(&self) -> String {
        format!("Goodbye to your Pet Dog: {}", self.name)
    }
}


pub fn bid_farewell<T: Farewell + Greet>(item: &T) {
    println!("{}", item.greet());
    println!("{}", item.farewell());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting() {
        let person = Person::new("Mark".into());
        let dog = Dog::new("Kelb".into());
        let robot = Robot::new(121);

        announce_greeting(&person);
        announce_greeting(&dog);
        announce_greeting(&robot);


        assert_eq!(person.greet(), "Welcome to Rustinun: Mark");
        assert_eq!(dog.greet(), "Welcome to your Pet Dog: Kelb");
        assert_eq!(robot.greet(), "Welcome to your bot: 121")
    }

    #[test]
    fn test_farewell() {
        let person = Person::new("Dawn".into());
        let dog = Dog::new("Belk".into());
        // let robot = Robot::new(131);

        bid_farewell(&person);
        bid_farewell(&dog);
        // bid_farewell(&robot);


        assert_eq!(person.farewell(), "See you later Dawn");
        assert_eq!(dog.farewell(), "Goodbye to your Pet Dog: Belk");
    }
}
