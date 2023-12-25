// Behavioral Pattern - Strategy
// This example demonstrates how the Strategy pattern allows for interchangeable algorithms by
// encapsulating them in separate strategy objects and providing a unified interface (DriveStrategy)
// for using different strategies interchangeably.

// This has to be used when Base class (B1) has a function (f1) and 2 Derived classes of (B1) i.e, (C1) and (C2) override (f1) say with same logic (f1').

// Here drive method is duplicated in SportsVehicle and OffRoadVehicle class whereas PassengerVehicle inherits from Base class
// class Vehicle {
//     public void drive() {
//         System.out.println!("Normal drive logic");
//     }
// }

// class SportsVehicle extends Vehicle {
//     public void drive() {
//         System.out.println!("Special drive logic");
//     }
// }

// class PassengerVehicle extends Vehicle { }

// class OffRoadVehicle extends Vehicle {
//     public void drive() {
//         System.out.println!("Special drive logic");
//     }
// }

// To avoid duplicacy of code use Strategy Pattern, Move drive function to interface and create 2 class with Special and Normal implementation for drive
// Later create child classes from latter classes.
interface DriveStrategy {
    public void drive();
}

class Vehicle {
    DriveStrategy obj;

    Vehicle(DriveStrategy obj) {
        this.obj = obj;
    }

    public void drive() {
        obj.drive();
    }
}

// Duplicate code in one function now
class SpecialVehicle implements DriveStrategy {
    public void drive() {
        System.out.println("Special drive logic");
    }
}

class NormalVehicle implements DriveStrategy {
    public void drive() {
        System.out.println("Normal drive logic");
    }
}

class SportsVehicle extends Vehicle { 
    SportsVehicle() {
        super(new SpecialVehicle());
    }
};

class PassengerVehicle extends Vehicle { 
    PassengerVehicle() {
        super(new NormalVehicle());
    }
};

class OffRoadVehicle extends Vehicle {
    OffRoadVehicle() {
        super(new SpecialVehicle());
    }
 };

class Main {
    public static void main(String args[]) {
        Vehicle sports = new SportsVehicle();
        Vehicle passenger = new PassengerVehicle();
        Vehicle off_road = new SportsVehicle();

        sports.drive();
        passenger.drive();
        off_road.drive();
    }
}

// //////////////////////////////////////////////////////////////////////////////////////////////////

// Behavoiral Pattern - Observer
// The Observer design pattern allows for a loosely coupled relationship between objects. The
// subject (observable) maintains a list of observers and notifies them when changes occur,
// without having explicit knowledge of the observers' concrete implementations. This promotes
// modularity and enables a flexible notification mechanism.


// Problem statement - Clients can click on notify_me on various products to notify them whenever there's a stock available. Also they can be notified using email or mobile

import java.util.*;
import java.io.*;

interface StockObservable {
    public void register_observer(StockObserver observer);

    public void notify_observer();

    public void set_stock(int new_stock);
}

class IphoneObservable implements StockObservable {
    List<StockObserver> observers = new ArrayList<StockObserver>();
    int number_of_items_left = 0;

    public void register_observer(StockObserver observer) {
        observers.add(observer);
    }

    public void notify_observer() {
        for(StockObserver observer:observers) {
            observer.update();
        }
    }

    public void set_stock(int new_stock) {
        if(number_of_items_left == 0) {
            notify_observer();
        }
        number_of_items_left += new_stock;
    }
}

interface StockObserver {
    public void update();
}

class MobileObserver implements StockObserver {
    String mobile_no;

    MobileObserver(String mobile_no) {
        this.mobile_no = mobile_no;
    }
    public void update() {
        System.out.println("Message sent to " + mobile_no);
    }
}

class EmailObserver implements StockObserver {
    String email;

    EmailObserver(String email) {
        this.email = email;
    }
    public void update() {
        System.out.println("Email sent to " + email);
    }
}

class Main {
    public static void main(String args[]) {
        StockObservable iphone_observable = new IphoneObservable();
        StockObserver user1 = new MobileObserver("123");
        iphone_observable.register_observer(user1);
        StockObserver user2 = new EmailObserver("abc@gmail.com");
        iphone_observable.register_observer(user2);
        StockObserver user3 = new EmailObserver("xyz@gmail.com");
        iphone_observable.register_observer(user3);

        iphone_observable.set_stock(10);
    }
}

// //////////////////////////////////////////////////////////////////////////////////////////////////

// Structural Pattern - Decorator
// Decorator allows adding new behaviors to objects dynamically by
// placing them inside special wrapper objects, called decorators.

abstract class BasePizza {
    abstract int get_cost();
}

class Pizza1 extends BasePizza {
    int get_cost() {
        return 10;
    }
}

class Pizza2 extends BasePizza {
    int get_cost() {
        return 20;
    }
}

// ToppingsDecorator "is-a" BasePizza
abstract class ToppingsDecorator extends BasePizza { }

class Topping1 extends ToppingsDecorator {
    // ToppingsDecorator "has-a" BasePizza
    BasePizza pizza;

    Topping1(BasePizza pizza) {
        this.pizza = pizza;
    }

    int get_cost() {
        return pizza.get_cost() + 1;
    }
}

class Topping2 extends ToppingsDecorator {
    BasePizza pizza;

    Topping2(BasePizza pizza) {
        this.pizza = pizza;
    }

    int get_cost() {
        return pizza.get_cost() + 2;
    }
}

class Main {
    public static void main(String args[]) {
        BasePizza pizza = new Pizza1();
        System.out.println("Cost of plain pizza = " + pizza.get_cost());

        BasePizza topping1_pizza = new Topping1(new Pizza1());
        System.out.println("Cost of pizza with topping1 = " + topping1_pizza.get_cost());
        
        BasePizza topping1_2_pizza = new Topping2(new Topping1(new Pizza1()));
        System.out.println("Cost of pizza with topping1 + topping2 = " + topping1_2_pizza.get_cost());
    }
}

// //////////////////////////////////////////////////////////////////////////////////////////////////

// Creational Pattern - Factory
// The Factory design pattern allows us to encapsulate the object creation logic and provide a
// unified interface (create_shape method) for creating different types of objects. This provides
// flexibility and allows for easier maintenance and future extensibility.

interface Shape {
    public void draw();
}

class Circle implements Shape {
    public void draw() {
        System.out.println("Drawing cirle");
    }
}

class Triangle implements Shape {
    public void draw() {
        System.out.println("Drawing triangle");
    }
}

class ShapeFactory {
    Shape create_shape(String shape) {
        switch (shape) {
            case "circle":
                return new Circle();
            case "triangle":
                return new Triangle();
            default:
                return null;
        }
    }
}

class Main {
    public static void main(String args[]) {
        ShapeFactory factory = new ShapeFactory();
        Shape circle = factory.create_shape("circle");
        circle.draw();
        Shape triangle = factory.create_shape("triangle");
        triangle.draw();
    }
}

// //////////////////////////////////////////////////////////////////////////////////////////////////

// // Creational Pattern - Abstract Factory
// // The Abstract Factory Pattern is used to create families of related objects. It consists of two
// // main components: the abstract factory(GUIFactory) and the concrete factory(Windows,
// // Mac). The abstract factory defines the interface for creating a family of products,
// // while the concrete factory implements this interface to create specific products.

interface GUIFactory {
    public Button create_button();
    public CheckBox create_checkbox();
}

class Windows implements GUIFactory {
    public Button create_button() {
        return new WinButton();
    }
    
    public CheckBox create_checkbox() {
        return new WinCheckbox();
    }
}

class Mac implements GUIFactory {
    public Button create_button() {
        return new MacButton();
    }
    
    public CheckBox create_checkbox() {
        return new MacCheckbox();
    }
}


interface Button {
    public void click();
}

class WinButton implements Button {
    public void click() {
        System.out.println("Button clicked in Windows");
    }
}

class MacButton implements Button {
    public void click() {
        System.out.println("Button clicked in Mac");
    }
}

interface CheckBox {
    public void check_box();
}

class WinCheckbox implements CheckBox {
    public void check_box() {
        System.out.println("Check box created in Windows");
    }
}

class MacCheckbox implements CheckBox {
    public void check_box() {
        System.out.println("Check box created in Mac");
    }
}

class Application {
    GUIFactory os;
    Button button;
    CheckBox check_box;
    Application(GUIFactory os) {
        this.os = os;
    }
    
    public void create_UI() {
        button = os.create_button();
        check_box = os.create_checkbox();
    }
    
    public void click_button() {
        button.click();
    }
    
    public void create_checkbox() {
        check_box.check_box();
    }
}

class Main {
    public static void main(String args[]) {
        GUIFactory windows = new Windows();
        Application app = new Application(windows);
        app.create_UI();
        app.click_button();
        app.create_checkbox();
    }
}

// //////////////////////////////////////////////////////////////////////////////////////////////////

