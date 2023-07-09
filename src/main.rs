// 1. Creational Pattern - Factory
// The Factory design pattern allows us to encapsulate the object creation logic and provide a
// unified interface (create_animal method) for creating different types of objects. This provides
// flexibility and allows for easier maintenance and future extensibility.

// trait Animal {
//     fn speak(&self);
// }

// struct Dog;
// struct Cat;

// impl Animal for Dog {
//     fn speak(&self) {
//         println!("Dog says: Woof!");
//     }
// }

// impl Animal for Cat {
//     fn speak(&self) {
//         println!("Cat says: Meow!");
//     }
// }

// enum AnimalType {
//     Dog,
//     Cat,
// }

// struct AnimalFactory;

// impl AnimalFactory {
//     fn create_animal(animal_type: AnimalType) -> Box<dyn Animal> {
//         match animal_type {
//             AnimalType::Dog => Box::new(Dog),
//             AnimalType::Cat => Box::new(Cat),
//         }
//     }
// }

// fn main() {
//     let dog = AnimalFactory::create_animal(AnimalType::Dog);
//     dog.speak();

//     let cat = AnimalFactory::create_animal(AnimalType::Cat);
//     cat.speak();
// }

// 2. Creational Pattern - Builder
// The Builder design pattern allows for a more flexible and readable way of constructing complex
// objects. It provides an interface for step-by-step construction while allowing the customization
// of individual properties

// #[derive(Default, Debug)]
// struct Burger {
//     item1: String,
//     item2: String,
// }

// #[derive(Debug, Default)]
// struct BurgerBuilder {
//     burger: Burger
// }

// impl BurgerBuilder {
//     fn new() -> Self {
//         BurgerBuilder::default()
//     }

//     fn add_item1(mut self, item1: String) -> Self {
//         self.burger.item1 = item1;
//         self
//     }

//     fn add_item2(mut self, item2: String) -> Self {
//         self.burger.item2 = item2;
//         self
//     }

//     fn build(self) -> Burger {
//         self.burger
//     }
// }
// fn main() {
//     let b1 = BurgerBuilder::new()
//         .add_item1("Item1".to_string())
//         .add_item2("Item2".to_string())
//         .build();
//     println!("{b1:#?}");
// }

// 3. Behavioral Pattern - Observer (PubSub)
// The Observer design pattern allows for a loosely coupled relationship between objects. The
// subject (observable) maintains a list of observers and notifies them when changes occur,
// without having explicit knowledge of the observers' concrete implementations. This promotes
// modularity and enables a flexible notification mechanism.

// trait YoutubeSubscriber {
//     fn send_notification(&self, event: String);
// }

// #[allow(dead_code)]
// struct YoutubeChannel {
//     name: String,
//     subscribers: Vec<Box<dyn YoutubeSubscriber>>
// }

// impl YoutubeChannel {
//     fn new(name: String) -> Self {
//         Self { name, subscribers: Vec::new() }
//     }
//     fn subscribe(&mut self, subscriber: Box<dyn YoutubeSubscriber>) {
//         self.subscribers.push(subscriber);
//     }

//     fn notify(&self, event: String) {
//         for sub in &self.subscribers {
//             sub.send_notification(event.clone());
//         }
//     }
// }

// struct YoutubeUser{
//     name: String
// }

// impl YoutubeSubscriber for YoutubeUser {
//     fn send_notification(&self, event: String) {
//         println!("User {} received notification: {}",self.name, event)
//     }
// }
// fn main() {
//     let mut channel = YoutubeChannel::new("funtime".to_string());
//     channel.subscribe(Box::new(YoutubeUser{name: "sub1".to_string()}));
//     channel.subscribe(Box::new(YoutubeUser{name: "sub2".to_string()}));
//     channel.subscribe(Box::new(YoutubeUser{name: "sub3".to_string()}));

//     channel.notify("New video uploaded".to_string());

// }

// 4. Behavioral Pattern - Strategy
// This example demonstrates how the Strategy pattern allows for interchangeable algorithms by
// encapsulating them in separate strategy objects and providing a unified interface (FilterStrategy)
// for using different strategies interchangeably.

// trait FilterStrategy {
//     fn remove_value(&self, val: i32) -> bool;
// }

// struct RemoveNegativeStrategy;

// impl FilterStrategy for RemoveNegativeStrategy {
//     fn remove_value(&self, val: i32) -> bool {
//         val >= 0
//     }
// }

// struct RemoveOddStrategy;

// impl FilterStrategy for RemoveOddStrategy {
//     fn remove_value(&self, val: i32) -> bool {
//         val % 2 == 0
//     }
// }

// #[derive(Debug)]
// struct Values(Vec<i32>);

// impl Values {
//     fn filter(&mut self, strategy: impl FilterStrategy) {
//         self.0.retain(|val| strategy.remove_value(*val));
//     }
// }

// fn main() {
//     let mut values = Values(vec![-1, 3, 2, 4, -5]);
//     values.filter(RemoveNegativeStrategy);
//     println!("{values:?}");
// }
