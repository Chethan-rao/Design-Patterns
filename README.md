<p align="center">
  <h1 align="center">
    Design Patterns
  </h1>
</p>

## Creational Patterns

| Pattern | Description |
|:-------:|:----------- |
| [Factory Method](https://github.com/Chethan-rao/Design-Patterns-Rust/blob/6238b06419d6444dbcefd6caf77cf1cf99ca4e18/src/main.rs#L7C32-L7C32) | Defers instantiation of an object to a specialized function for creating instances |
| [Abstract Factory](https://github.com/Chethan-rao/Design-Patterns-Rust/blob/6238b06419d6444dbcefd6caf77cf1cf99ca4e18/src/main.rs#L55) | Provides an interface for creating families of releated objects |
| [Builder](https://github.com/Chethan-rao/Design-Patterns-Rust/blob/6238b06419d6444dbcefd6caf77cf1cf99ca4e18/src/main.rs#L180) | Builds a complex object using simple objects |
| [Singleton](https://github.com/Chethan-rao/Design-Patterns-Rust/blob/6238b06419d6444dbcefd6caf77cf1cf99ca4e18/src/main.rs#L250) | Restricts instantiation of a type to one object |
| [Prototype](https://github.com/Chethan-rao/Design-Patterns-Rust/blob/6238b06419d6444dbcefd6caf77cf1cf99ca4e18/src/main.rs#L274) | Restricts instantiation of a type to one object |


## Behavioral Patterns
| Pattern | Description |
|:-------:|:----------- |
| [Strategy](/behavioral/strategy.rs) | Enables an algorithm's behavior to be selected at runtime |
| [State](/behavioral/state.rs) | Encapsulates varying behavior for the same object based on its internal state |
| [Command](/behavioral/command.rs) | Converts requests or simple operations into objects. |
| [Iterator](/behavioral/iterator.rs) |  Lets you traverse elements of a collection without exposing its underlying representation |
| [Observer](/behavioral/observer.rs) | Allows one objects to notify other objects about changes in their state. |
| [Chain of Responsibility](/behavioral/chain_of_responsibility.rs) | Avoids coupling a sender to receiver by giving more than object a chance to handle the request |


## Structural Patterns

| Pattern | Description |
|:-------:|:----------- |
| [Adapter](https://github.com/Chethan-rao/Design-Patterns-Rust/blob/6238b06419d6444dbcefd6caf77cf1cf99ca4e18/src/main.rs#L407) | allows objects with incompatible interfaces to collaborate. |
| [Composite](https://github.com/Chethan-rao/Design-Patterns-Rust/blob/6238b06419d6444dbcefd6caf77cf1cf99ca4e18/src/main.rs#L617) | lets you compose objects into tree structures |
| [Decorator](https://github.com/Chethan-rao/Design-Patterns-Rust/blob/6238b06419d6444dbcefd6caf77cf1cf99ca4e18/src/main.rs#L688) | Adds behavior to an object, statically or dynamically |
| [Facade](https://github.com/Chethan-rao/Design-Patterns-Rust/blob/6238b06419d6444dbcefd6caf77cf1cf99ca4e18/src/main.rs#L754) | Provides a simple interface before a complex system |
| [FlyWeight](https://github.com/Chethan-rao/Design-Patterns-Rust/blob/6238b06419d6444dbcefd6caf77cf1cf99ca4e18/src/main.rs#L810) | Lets you fit more objects into the available RAM by sharing common parts of state between multiple objects |
| [Proxy](https://github.com/Chethan-rao/Design-Patterns-Rust/blob/6238b06419d6444dbcefd6caf77cf1cf99ca4e18/src/main.rs#L921) | Provides a surrogate for an object to control it's actions |
