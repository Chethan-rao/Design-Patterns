// // Creational patterns are primarily concerned with object creation and instantiation.
// // Structural patterns are concerned with the composition of objects to create larger structures.
// // Behavioral patterns deal with the communication and collaboration between objects to achieve specific behaviors.

// //////////////////////////////////////////////////////////////////////////////////////////////////

// // Creational Pattern - Factory
// // The Factory design pattern allows us to encapsulate the object creation logic and provide a
// // unified interface (create_animal method) for creating different types of objects. This provides
// // flexibility and allows for easier maintenance and future extensibility.

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

// // Factory Method
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

// //////////////////////////////////////////////////////////////////////////////////////////////////

// // Creational Pattern - Abstract Factory
// // The Abstract Factory Pattern is used to create families of related objects. It consists of two
// // main components: the abstract factory(GUIFactory) and the concrete factory(WinFactory,
// // LinuxFactory). The abstract factory defines the interface for creating a family of products,
// // while the concrete factory implements this interface to create specific products.

// trait GUIFactory {
//     fn create_button(&self) -> Box<dyn Button>;
//     fn create_menu(&self) -> Box<dyn Menu>;
//     fn create_textbox(&self) -> Box<dyn Textbox>;
// }

// trait Button {
//     fn paint(&self);
// }

// trait Menu {
//     fn display(&self);
// }

// trait Textbox {
//     fn text(&self) -> String;
// }

// struct WinFactory {}

// impl GUIFactory for WinFactory {
//     fn create_button(&self) -> Box<dyn Button> {
//         Box::new(WinButton {})
//     }

//     fn create_menu(&self) -> Box<dyn Menu> {
//         Box::new(WinMenu {})
//     }

//     fn create_textbox(&self) -> Box<dyn Textbox> {
//         Box::new(WinTextbox {})
//     }
// }

// struct LinuxFactory {}

// impl GUIFactory for LinuxFactory {
//     fn create_button(&self) -> Box<dyn Button> {
//         Box::new(LinuxButton {})
//     }

//     fn create_menu(&self) -> Box<dyn Menu> {
//         Box::new(LinuxMenu {})
//     }

//     fn create_textbox(&self) -> Box<dyn Textbox> {
//         Box::new(LinuxTextbox {})
//     }
// }

// struct WinButton {}

// impl Button for WinButton {
//     fn paint(&self) {
//         println!("Windows Button")
//     }
// }

// struct LinuxButton {}

// impl Button for LinuxButton {
//     fn paint(&self) {
//         println!("Linux Button")
//     }
// }

// struct WinMenu {}

// impl Menu for WinMenu {
//     fn display(&self) {
//         println!("Windows Menu")
//     }
// }

// struct LinuxMenu {}

// impl Menu for LinuxMenu {
//     fn display(&self) {
//         println!("Linux Menu")
//     }
// }

// struct WinTextbox {}

// impl Textbox for WinTextbox {
//     fn text(&self) -> String {
//         String::from("Windows Textbox")
//     }
// }

// struct LinuxTextbox {}

// impl Textbox for LinuxTextbox {
//     fn text(&self) -> String {
//         String::from("Linux Textbox")
//     }
// }

// // Factory method
// fn create_gui(factory: &dyn GUIFactory) {
//     let button = factory.create_button();
//     let menu = factory.create_menu();
//     let textbox = factory.create_textbox();

//     button.paint();
//     menu.display();
//     println!("{}", textbox.text());
// }

// fn main() {
//     let win_factory = WinFactory {};
//     let linux_factory = LinuxFactory {};

//     create_gui(&win_factory);
//     create_gui(&linux_factory);
// }

// //////////////////////////////////////////////////////////////////////////////////////////////////

// // Creational Pattern - Builder
// // The Builder design pattern allows for a more flexible and readable way of constructing complex
// // objects. It provides an interface for step-by-step construction while allowing the customization
// // of individual properties
// // Instead of creating multiple constructor with variable number of arguments, use this pattern

// #[derive(Debug)]
// pub struct KubernetesCluster {
//     name: String,
//     version: String,
//     auto_upgrade: bool,
//     node_pool: Option<String>,
// }

// pub struct KubernetesClusterBuilder {
//     name: String,
//     version: String,
//     auto_upgrade: Option<bool>,
//     node_pool: Option<String>,
// }

// impl KubernetesClusterBuilder {
//     fn new(name: String, version: String) -> KubernetesClusterBuilder {
//         KubernetesClusterBuilder {
//             name,
//             version,
//             auto_upgrade: None,
//             node_pool: None,
//         }
//     }
//     fn auto_upgrade(&mut self, auto_upgrade: bool) -> &mut Self {
//         self.auto_upgrade = Some(auto_upgrade);
//         self
//     }

//     fn node_pool(&mut self, node_pool: String) -> &mut Self {
//         self.node_pool = Some(node_pool);
//         self
//     }

//     fn build(&mut self) -> KubernetesCluster {
//         KubernetesCluster {
//             name: self.name.clone(),
//             version: self.version.clone(),
//             auto_upgrade: self.auto_upgrade.unwrap_or_default(),
//             node_pool: self.node_pool.clone(),
//         }
//     }
// }

// fn main() {
//     let name = "my-cluster".to_owned();
//     let version = "1.25.0".to_owned();

//     let nodes = "Node1".to_string();

//     let _basic_cluster = KubernetesClusterBuilder::new(name.clone(), version.clone()).build();

//     let _auto_upgrade_cluster = KubernetesClusterBuilder::new(name.clone(), version.clone())
//         .auto_upgrade(true)
//         .build();

//     let _complete_cluster = KubernetesClusterBuilder::new(name.clone(), version.clone())
//         .auto_upgrade(true)
//         .node_pool(nodes)
//         .build();
// }

// //////////////////////////////////////////////////////////////////////////////////////////////////

// // Creational Pattern - Singleton (In Java)
// // This pattern involves a single class (called Singleton) which is responsible to create an object
// // while making sure that only single object(instance) gets created. This class provides a way to
// // access its only object which can be accessed directly using getInstance() method without need
// // to instantiate the object of the class which should create one if there isn't one created already.

// // Thread Synchronized Java implementation of singleton design pattern
// class Singleton
// {
// 	private static Singleton obj;
//     //make the constructor private so that this class cannot be instantiated
// 	private Singleton() {}

// 	// Only one thread can execute this at a time and this is the only way to instantiate
// 	public static synchronized Singleton getInstance()
// 	{
// 		if (obj==null)
// 			obj = new Singleton();
// 		return obj;
// 	}
// }

// //////////////////////////////////////////////////////////////////////////////////////////////////

// // Creational Pattern - Prototype
// // All prototype classes should have a common interface that makes it possible to copy objects

// trait Prototype {
//     fn get_clone(&self) -> Self;
// }

// #[derive(Debug)]
// struct Human {
//     name: String,
//     age: i32,
// }

// impl Prototype for Human {
//     fn get_clone(&self) -> Self {
//         Self {
//             name: self.name.clone(),
//             age: self.age,
//         }
//     }
// }

// fn main() {
//     let mut human1 = Human {
//         name: "Chethan".to_string(),
//         age: 21,
//     };

//     let human2 = human1.get_clone();

//     human1.age = 22;

//     println!("{human1:?} {human2:?}");
// }

// //////////////////////////////////////////////////////////////////////////////////////////////////

// // Structural Pattern - Adapter
// // The Adapter acts as a wrapper between two objects. It catches calls for one object and
// // transforms them to format and interface recognizable by the second object.
// // Main intention is not to implement the Target trait to InCompatible

// // Example 1 -

// pub trait Target {
//     fn request(&self) -> String;
// }

// fn call(target: impl Target) {
//     println!("{}", target.request());
// }

// pub struct Compatible;

// impl Compatible {
//     fn specific_request(&self) -> String {
//         "I'm comptaible object".to_string()
//     }
// }

// impl Target for Compatible {
//     fn request(&self) -> String {
//         self.specific_request()
//     }
// }

// // Adaptee - Object which is incompatible
// pub struct InCompatible;

// impl InCompatible {
//     fn specific_request(&self) -> String {
//         "I'm incomptaible object".to_string()
//     }
// }

// // Object which makes incompatible objects as compatible
// pub struct Adapter {
//     adaptee: InCompatible,
// }

// impl Target for Adapter {
//     fn request(&self) -> String {
//         self.adaptee.specific_request()
//     }
// }

// fn main() {
//     call(Compatible);

//     // Gives error as this is incompatible (doesn't want to implement Target trait)
//     // let incompatible = call(InCompatible);

//     // So build a wrapper around InCompatible
//     let adaptor = Adapter {
//         adaptee: InCompatible,
//     };
//     call(adaptor);
// }

// // Example 2 -

// /*
//  * Core Trait that defines a basic Rocket Ship
//  */
// trait RocketShip {
//     fn turn_on(&self);
//     fn turn_off(&self);
//     fn blast_off(&self);
//     fn fly(&self);
// }

// /*
//  * Basic struct for a NASA Ship
//  */
// struct NASAShip;

// /*
//  * Implement RocketShip trait to add functionality to NASAShip
//  */
// impl RocketShip for NASAShip {
//     fn turn_on(&self) {
//         println!("NASA Ship is turning on.")
//     }

//     fn turn_off(&self) {
//         println!("NASA Ship is turning off.")
//     }

//     fn blast_off(&self) {
//         println!("NASA Ship is blasting off.")
//     }

//     fn fly(&self) {
//         println!("NASA Ship is flying away.")
//     }
// }

// /*
//  * Uh oh, here is our problem. It's the amazingly advanced SpaceX ship that our
//  * astronaut doesn't know how to pilot.
//  */
// trait SpaceXShip {
//     fn ignition(&self);
//     fn on(&self);
//     fn off(&self);
//     fn launch(&self);
//     fn fly(&self);
// }

// /*
//  * Basic struct for a SpaceX Dragon rocket ship
//  */
// struct SpaceXDragon;

// /*
//  * Implement the SpaceX trait to add functionality to the Space X Dragon
//  */
// impl SpaceXShip for SpaceXDragon {
//     fn ignition(&self) {
//         println!("Turning Dragon's ignition.")
//     }

//     fn on(&self) {
//         println!("Turning on the Dragon.")
//     }

//     fn off(&self) {
//         println!("Turning off the Dragon.")
//     }

//     fn launch(&self) {
//         println!("Launching the Dragon")
//     }

//     fn fly(&self) {
//         println!("The Dragon is flying away.")
//     }
// }

// /*
//  * Uh oh, the new SpaceXDragon doesn't implement the RocketShip interface. We
//  * need to create an adapter that does.
//  */
// /*
//  * Adapter to adapt anything that implements SpaceXShip to the RocketShip trait
//  */
// struct SpaceXAdapter {
//     ship: SpaceXDragon,
// }

// /*
//  * SpaceX Adapter that adds RocketShip traits to any SpaceXShip
//  */
// impl RocketShip for SpaceXAdapter {
//     fn turn_on(&self) {
//         self.ship.ignition();
//         self.ship.on();
//     }

//     fn turn_off(&self) {
//         self.ship.off();
//     }

//     fn blast_off(&self) {
//         self.ship.launch();
//     }

//     fn fly(&self) {
//         self.ship.fly();
//     }
// }

// /*
//  * Basic function to pilot ships that implement the RocketShip trait
//  */
// fn pilot(ship: &impl RocketShip) {
//     ship.turn_on();
//     ship.blast_off();
//     ship.fly();
//     ship.turn_off();
//     println!("\n");
// }

// fn main() {
//     // Create a new NASAShip
//     let saturn5 = NASAShip;

//     // Let's fly our NASAShip
//     println!("Piloting the Saturn 5.");
//     pilot(&saturn5);

//     // Create a Dragon
//     let dragon = SpaceXDragon;

//     // Uh oh, our pilot function doesn't recognize this ship...
//     // pilot(&dragon); <-- Gives a compile time error.

//     // Let's Adapt our SpaceXDragon ship
//     let dragon_adapter = SpaceXAdapter { ship: dragon };

//     // Now we can pilot the Dragon!
//     println!("Piloting the Dragon Adapter.");
//     pilot(&dragon_adapter);
// }

// //////////////////////////////////////////////////////////////////////////////////////////////////

// // Structural Pattern - Composite
// // Composite is a structural design pattern that lets you compose objects into tree structures
// // and then work with these structures as if they were individual objects.

// pub trait Component {
//     fn search(&self, keyword: &str);
// }

// struct File {
//     name: &'static str,
// }

// impl File {
//     fn new(name: &'static str) -> Self {
//         Self { name }
//     }
// }

// impl Component for File {
//     fn search(&self, keyword: &str) {
//         println!("Searching for {} in file {}", keyword, self.name);
//     }
// }

// struct Folder {
//     name: &'static str,
//     inner_components: Vec<Box<dyn Component>>,
// }

// impl Folder {
//     fn new(name: &'static str) -> Self {
//         Self {
//             name,
//             inner_components: vec![],
//         }
//     }

//     fn insert_component(&mut self, component: impl Component + 'static) {
//         self.inner_components.push(Box::new(component));
//     }
// }

// impl Component for Folder {
//     fn search(&self, keyword: &str) {
//         println!(
//             "Searching recursively for keyword {} in folder {}",
//             keyword, self.name
//         );

//         for component in self.inner_components.iter() {
//             component.search(keyword);
//         }
//     }
// }

// fn main() {
//     let file1 = File::new("file1");
//     let file2 = File::new("file2");
//     let file3 = File::new("file3");

//     let mut folder1 = Folder::new("folder1");
//     let mut folder2 = Folder::new("folder2");

//     folder1.insert_component(file1);
//     folder2.insert_component(file2);
//     folder2.insert_component(file3);
//     folder2.insert_component(folder1);

//     folder2.search("rose");
// }

// //////////////////////////////////////////////////////////////////////////////////////////////////

// // Structural Pattern - Decorator (In Java)
// // Decorator is a structural pattern that allows adding new behaviors to objects dynamically by
// // placing them inside special wrapper objects, called decorators.

// interface Color{
//     void fill();
// }

// //concrete component of the base interface
// class Black implements Color{
//     @Override
//     public void fill(){
//         System.out.println("Black color");
//     }
// }

// //abstract decorator class
// abstract class ColorDecorator implements Color{

//     //base class object
//     protected Color colored;

//     //constructor with base class object as the parameter
//     public ColorDecorator(Color colored){
//         this.colored = colored;
//     }

//     public void fill(){
//         colored.fill();
//     }
// }

// //concrete decorator class extending base decorator class
// class PatternDecorator extends ColorDecorator{
//     public PatternDecorator(Color colored){
//         super(colored);
//     }

//     public void fill(){
//         colored.fill();
//         addPattern(colored);
//     }

//     private void addPattern(Color colored){
//         System.out.println("Pattern");
//     }
// }

// class Demo{
//     public static void main(String[] args){

//     Color black = new Black();
//     Color pattern = new PatternDecorator(new Black());

//     System.out.println("\nStyle: Solid");
//     black.fill();

//     System.out.println("\nStyle: Pattern");
//     pattern.fill();
//     }
// }

// //////////////////////////////////////////////////////////////////////////////////////////////////

// // Structural Pattern - Facade
// // Facade design pattern is a type of structural design pattern. The main objective of facade
// // design pattern is to provide a simple interface before a complex system. So that the user or
// // client can access the simple interface without knowing the complexity of its sub-system.
// // Facade design patterns are mostly used to hide the dependencies involved in a system from the user.

// struct PlaceOrder;

// impl PlaceOrder {
//     fn place_order(&self) {
//         println!("Order placed");
//     }
// }

// struct Payment;

// impl Payment {
//     fn make_payment(&self) {
//         println!("Payment received");
//     }
// }

// struct Delivery;

// impl Delivery {
//     fn delivery(&self) {
//         println!("Order Delivered");
//     }
// }

// // Facade
// struct Operation {
//     order: PlaceOrder,
//     payment: Payment,
//     delivery: Delivery,
// }

// impl Operation {
//     fn complete_order(&self) {
//         self.order.place_order();
//         self.payment.make_payment();
//         self.delivery.delivery();
//     }
// }

// fn main() {
//     let operation = Operation {
//         order: PlaceOrder,
//         payment: Payment,
//         delivery: Delivery,
//     };
//     operation.complete_order();
// }

// //////////////////////////////////////////////////////////////////////////////////////////////////

// // Structural Pattern - FlyWeight
// // Lets you fit more objects into the available RAM by sharing common parts of state between
// // multiple objects, instead of storing all of the data in each object individually.

// // Consider there are only 2 types of book and depending on type, distributor also change.
// // Hence we have limited options for both type_of_book and distributor.

// // struct Book {
// //     name: String,
// //     prize: i32,
// //     type_of_book: String,
// //     distributor: String,
// // }

// // So split them into separate struct

// use std::collections::HashMap;

// struct Book {
//     name: String,
//     prize: i32,
//     book_type: BookType,
// }

// #[derive(Clone)]
// struct BookType {
//     type_of_book: String,
//     distributor: String,
// }

// // FlyWeight Factory (Like a cache)
// struct BookFactory {
//     book_type: HashMap<String, BookType>,
// }

// impl BookFactory {
//     fn get_book_type(&mut self, type_of_book: String, distributor: String) -> BookType {
//         let book_type = self.book_type.get(&type_of_book);
//         if book_type.is_none() {
//             self.book_type.insert(
//                 type_of_book.clone(),
//                 BookType {
//                     type_of_book: type_of_book.clone(),
//                     distributor,
//                 },
//             );
//         }
//         self.book_type.get(&type_of_book).cloned().unwrap()
//     }
// }

// struct Store {
//     books: Vec<Book>,
// }

// impl Store {
//     fn add_book(
//         &mut self,
//         book_factory: &mut BookFactory,
//         name: String,
//         prize: i32,
//         type_of_book: String,
//         distributor: String,
//     ) {
//         let book_type = book_factory.get_book_type(type_of_book, distributor);
//         let book = Book {
//             name,
//             prize,
//             book_type
//         };
//         self.books.push(book);
//     }

//     fn display_books(&self) {
//         self.books.iter().for_each(|book| {
//             let r = format!(
//                 "{} {} {} {}",
//                 book.name, book.prize, book.book_type.type_of_book, book.book_type.distributor
//             );
//             println!("{r}");
//         });
//     }
// }

// fn main() {
//     let mut store = Store { books: vec![] };
//     let mut book_factory = BookFactory {
//         book_type: HashMap::new(),
//     };
//     for i in 0..5 {
//         store.add_book(
//             &mut book_factory,
//             format!("book{}", i + 1),
//             i + 10,
//             "Action".to_string(),
//             "distributor1".to_string(),
//         );
//         store.add_book(
//             &mut book_factory,
//             format!("book{}", i + 2),
//             i + 20,
//             "Adventure".to_string(),
//             "distributor2".to_string(),
//         )
//     }

//     store.display_books();
// }

// //////////////////////////////////////////////////////////////////////////////////////////////////

// // Structural Pattern - Proxy
// // Proxy is a structural design pattern that provides an object that acts as a substitute for a
// // real service object used by a client. A proxy receives client requests, does some work (access
// // control, caching, etc.) and then passes the request to a service object.

// use std::collections::HashMap;

// pub trait Server {
//     fn handle_request(&mut self, url: &str, method: &str) -> (u16, String);
// }

// pub struct Application;

// impl Server for Application {
//     fn handle_request(&mut self, url: &str, method: &str) -> (u16, String) {
//         if url == "/app/status" && method == "GET" {
//             return (200, "Ok".into());
//         }

//         if url == "/create/user" && method == "POST" {
//             return (201, "User Created".into());
//         }

//         (404, "Not Ok".into())
//     }
// }

// /// NGINX server is a proxy to an application server.
// pub struct NginxServer {
//     application: Application,
//     max_allowed_requests: u32,
//     rate_limiter: HashMap<String, u32>,
// }

// impl NginxServer {
//     pub fn new() -> Self {
//         Self {
//             application: Application,
//             max_allowed_requests: 2,
//             rate_limiter: HashMap::default(),
//         }
//     }

//     pub fn check_rate_limiting(&mut self, url: &str) -> bool {
//         let rate = self.rate_limiter.entry(url.to_string()).or_insert(1);

//         if *rate > self.max_allowed_requests {
//             return false;
//         }

//         *rate += 1;
//         true
//     }
// }

// impl Server for NginxServer {
//     fn handle_request(&mut self, url: &str, method: &str) -> (u16, String) {
//         if !self.check_rate_limiting(url) {
//             return (403, "Not Allowed".into());
//         }

//         self.application.handle_request(url, method)
//     }
// }

// fn main() {
//     let app_status = &"/app/status".to_string();
//     let create_user = &"/create/user".to_string();

//     // Instead of calling handle_request through Application, we call it through Nginx
//     let mut nginx = NginxServer::new();

//     let (code, body) = nginx.handle_request(app_status, "GET");
//     println!("Url: {}\nHttpCode: {}\nBody: {}\n", app_status, code, body);

//     let (code, body) = nginx.handle_request(app_status, "GET");
//     println!("Url: {}\nHttpCode: {}\nBody: {}\n", app_status, code, body);

//     let (code, body) = nginx.handle_request(app_status, "GET");
//     println!("Url: {}\nHttpCode: {}\nBody: {}\n", app_status, code, body);

//     let (code, body) = nginx.handle_request(create_user, "POST");
//     println!("Url: {}\nHttpCode: {}\nBody: {}\n", create_user, code, body);

//     let (code, body) = nginx.handle_request(create_user, "GET");
//     println!("Url: {}\nHttpCode: {}\nBody: {}\n", create_user, code, body);
// }

// //////////////////////////////////////////////////////////////////////////////////////////////////

// // Behavioral Pattern - Observer (PubSub)
// // The Observer design pattern allows for a loosely coupled relationship between objects. The
// // subject (observable) maintains a list of observers and notifies them when changes occur,
// // without having explicit knowledge of the observers' concrete implementations. This promotes
// // modularity and enables a flexible notification mechanism.

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

// //////////////////////////////////////////////////////////////////////////////////////////////////

// // Behavioral Pattern - Strategy
// // This example demonstrates how the Strategy pattern allows for interchangeable algorithms by
// // encapsulating them in separate strategy objects and providing a unified interface (FilterStrategy)
// // for using different strategies interchangeably.

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

// //////////////////////////////////////////////////////////////////////////////////////////////////
// // Behavioral Pattern - Chain of Responsibility
// // It allows passing request along the chain of potential handlers to handle a request generated
// // by a client.
// // Can scale well as we just have to had new independent handler and implement Department trait
// // without touching exisiting code (More flexible, less rigid)

// #[derive(Default)]
// struct User {
//     order_placed: bool,
//     payment_done: bool,
//     delivered: bool,
// }
// trait Department {
//     fn execute(&mut self, user: &mut User) {
//         self.handle(user);
//         if let Some(next) = &mut self.next() {
//             next.execute(user);
//         }
//     }

//     fn handle(&mut self, user: &mut User);

//     fn next(&mut self) -> &mut Option<Box<dyn Department>>;
// }

// #[derive(Default)]
// struct Order {
//     next: Option<Box<dyn Department>>,
// }

// impl Department for Order {
//     fn handle(&mut self, user: &mut User) {
//         if user.order_placed {
//             println!("Order is already placed");
//         } else {
//             user.order_placed = true;
//             println!("Order placed");
//         }
//     }

//     fn next(&mut self) -> &mut Option<Box<dyn Department>> {
//         &mut self.next
//     }
// }

// #[derive(Default)]

// struct PaymentA {
//     next: Option<Box<dyn Department>>,
// }

// impl Department for PaymentA {
//     fn handle(&mut self, user: &mut User) {
//         if user.payment_done {
//             println!("Payment is already done");
//         } else {
//             user.payment_done = true;
//             println!("Payment done");
//         }
//     }

//     fn next(&mut self) -> &mut Option<Box<dyn Department>> {
//         &mut self.next
//     }
// }

// #[derive(Default)]

// struct DeliveryA {
//     next: Option<Box<dyn Department>>,
// }

// impl Department for DeliveryA {
//     fn handle(&mut self, user: &mut User) {
//         if user.delivered {
//             println!("Delivery is already done");
//         } else {
//             user.delivered = true;
//             println!("Delivered");
//         }
//     }

//     fn next(&mut self) -> &mut Option<Box<dyn Department>> {
//         &mut self.next
//     }
// }

// fn main() {
//     let mut user = User::default();

//     let delivery = DeliveryA { next: None };
//     let payment = PaymentA {
//         next: Some(Box::new(delivery)),
//     };
//     let mut order = Order {
//         next: Some(Box::new(payment)),
//     };

//     order.execute(&mut user);

//     println!("\nThe Order has been already handled:\n");

//     order.execute(&mut user);
// }

// //////////////////////////////////////////////////////////////////////////////////////////////////
// // Behavioral Pattern - Command
// // This pattern helps you combine the commands into an object and provide an unified function to
// // call the command

// // Each action is encapsulated into a struct with the trait Command

// use std::collections::HashMap;

// trait Command {
//     fn execute(&self);
// }

// #[derive(Copy, Clone)]
// struct TV;
// impl TV {
//     fn new() -> TV {
//         TV
//     }
//     fn on(&self) {
//         println!("TV is on, watch movies.");
//     }
//     fn off(&self) {
//         println!("TV is off");
//     }
// }

// struct TVOnCommand {
//     tv: TV,
// }

// impl TVOnCommand {
//     fn new(tv: TV) -> TVOnCommand {
//         TVOnCommand { tv }
//     }
// }

// impl Command for TVOnCommand {
//     fn execute(&self) {
//         self.tv.on();
//     }
// }

// struct TVOffCommand {
//     tv: TV,
// }

// impl TVOffCommand {
//     fn new(tv: TV) -> TVOffCommand {
//         TVOffCommand { tv }
//     }
// }

// impl Command for TVOffCommand {
//     fn execute(&self) {
//         self.tv.off();
//     }
// }

// struct TVRemoteControl {
//     commands: HashMap<i32, Box<dyn Command>>,
// }

// impl TVRemoteControl {
//     fn new() -> TVRemoteControl {
//         TVRemoteControl {
//             commands: HashMap::new(),
//         }
//     }
//     fn set_command(&mut self, idx: i32, cmd: Box<dyn Command>) {
//         self.commands.insert(idx, cmd);
//     }
//     fn press_button(&self, idx: i32) {
//         if let Some(cmd) = self.commands.get(&idx) {
//             cmd.execute();
//         } else {
//             println!("do nothing.");
//         }
//     }
// }

// fn main() {
//     let tv = TV::new();
//     let mut remote_control = TVRemoteControl::new();
//     remote_control.press_button(0);

//     remote_control.set_command(1, Box::new(TVOnCommand::new(tv)));
//     remote_control.set_command(2, Box::new(TVOffCommand::new(tv)));

//     remote_control.press_button(1);
//     remote_control.press_button(2);
// }

// //////////////////////////////////////////////////////////////////////////////////////////////////
// // Behavioral Pattern - Iterater
// // Iterator is a behavioral design pattern that lets you traverse elements of a collection without
// // exposing its underlying representation (list, stack, tree, etc.).

// trait Iterator<T> {
//     fn next(&mut self) -> Option<T>;
//     fn current(&self) -> Option<T>;
//     fn has_next(&self) -> bool;
//     fn reset(&mut self);
// }

// struct Container<T> {
//     data: Vec<T>,
// }

// struct ConcreteIterator<'a, T> {
//     idx: usize,
//     container: &'a Container<T>,
// }

// impl<'a, T: Clone> ConcreteIterator<'a, T> {
//     fn new(container: &'a Container<T>) -> ConcreteIterator<T> {
//         ConcreteIterator { idx: 0, container }
//     }
// }

// impl<'a, T: Clone> Iterator<T> for ConcreteIterator<'a, T> {
//     fn next(&mut self) -> Option<T> {
//         let item = self.container.data.get(self.idx).cloned();
//         self.idx += 1;
//         item
//     }
//     fn current(&self) -> Option<T> {
//         self.container.data.get(self.idx).cloned()
//     }
//     fn has_next(&self) -> bool {
//         self.container.data.len() > self.idx
//     }
//     fn reset(&mut self) {
//         self.idx = 0;
//     }
// }

// impl<T: Clone> Container<T> {
//     fn new() -> Container<T> {
//         Container { data: Vec::new() }
//     }
//     fn add_item(&mut self, item: T) {
//         self.data.push(item);
//     }
//     fn iter(&self) -> impl Iterator<T> + '_ {
//         ConcreteIterator::new(self)
//     }
// }

// fn main() {
//     let mut c = Container::new();
//     c.add_item(1);
//     c.add_item(2);
//     c.add_item(3);

//     let mut iter = c.iter();
//     let has_next = iter.has_next();
//     assert_eq!(has_next, true);
//     let item = iter.next();
//     println!("item: {:?}", item);
//     iter.reset();
//     while iter.has_next() {
//         let v = iter.next().unwrap();
//         println!("item: {}", v);
//     }
//     let item = iter.next();
//     assert_eq!(item, None);
// }

// //////////////////////////////////////////////////////////////////////////////////////////////////
// // Behavioral Pattern - Mediator
// // Mediator is a behavioral design pattern that reduces coupling between components of a program
// // by making them communicate indirectly, through a special mediator object.
// // A train gets a mediator object by reference.

// struct User {
//     name: String,
//     mediator: Box<dyn ChatMediator>,
// }

// impl User {
//     fn send(&self, message: String);
//     fn receive(&self, message: String);
// }
// trait ChatMediator {
//     fn add_user(&mut self, user: User);
//     fn send_message(&self, user: User, message: String);
// }

// struct ChatMediatorImpl {
//     users: Vec<User>,
// }

// impl ChatMediator for ChatMediatorImpl {
//     fn add_user(&mut self, user: User) {
//         self.users.push(user);
//     }
//     fn send_message(&self, user: User, message: String) {
//         todo!()
//     }
// }
