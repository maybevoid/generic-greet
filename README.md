# Generic Greet

This repository contains the example generic greet code for my upcoming blog post on generic programming in Rust.

## Preface

Rust has a powerful type system that allows us to build abstractions that are _well-typed_. That is, the Rust compiler can help us guarantee that once an abstraction is implemented, it can be safely used with _all_ specialized use of the abstraction that satisfy the imposed _invariants_.

In this post, we are going to learn some basic design patterns for doing _generic programming_ in Rust. For programmers coming from languages such as Java and C++, the use of generics may seem exotic and intimidating. By the end of this post, we hope that readers will get a better sense of what generic programming is about, and how it differs from object-oriented programmings (OOP).

## Generic Greet

For the purpose of demonstrating the programming techniques, we are going to use a _contrived_ example of implementing greet functions that will return strings that greet a person. In the simplest case, we can write a `greet` function as follows:

```rust
fn greet(name: &str) -> String
{
  format!("Hello, {}!", name)
}
```

Our `greet` function is pretty minimal, with it greeting "Hello" to a person's name specified in the string. This example is _deliberately_ chosen so that we can focus on the programming techniques, rather than getting distracted on whether to implement a greet function in the given way.

Hence for purpose of stirring the discussion: Yes, if what we _really_ want is to implement a greet function, the version of `greet` function above is already _sufficient_ for the purpose. Commenters reserve the right to _not_ recommend the use in production for the more complex `greet` functions that we will explore later.

That being said, let's think for a second a more realistic scenario of how the greet function _would_ be implemented in a real world application. In practice, even a simple `greet` function may contain complex logic such as:

- Formatting the output to be displayed in HTML or GUI.
- Greet a peron differently depending on who they are.
- Use a different language or theme depending on the person's preferences.

For the purpose of this post, we are going to _pretend_ that our greet functions are complex and implement the logic such as listed above.

## Greeting Person

Our initial `greet` function simply takes in a person's name as a string. What if it needs more information than just a person's name? In practice, we would define _data types_ such as a `CasualPerson` struct to hold all information about a person:

```rust
pub struct CasualPerson
{
  pub name: String,
  // additional fields omitted
}
```

Our `CasualPerson` struct may contain other information such as email address and phone number. For the purpose of the `greet` function, we only care about the person's name, so we have the other fields omitted. Our `greet` function is then updated to take in `&CasualPerson` as a parameter:

```rust
fn greet(person: &CasualPerson) -> String
{
  format!("Hello, {}!", person.name)
}
```

By taking in the full `CasualPerson` struct, our `greet` function will be able to be _extended_ in the future to make use of other fields, such as whether to greet the person in dark mode.

To make it easier to construct new `CasualPerson` values, we also define a `new` constructor method:

```rust
impl CasualPerson
{
  pub fn new(name: &str) -> Self
  {
    Self {
      name: name.to_string(),
    }
  }
}
```

## Greeting Different Kinds of Persons

So far we have a single `CasualPerson` struct that needs to be greeted. But what if our application has multiple structs that the `greet` function needs to handle? For example, we may have a `FormalPerson` struct that records the first and last name as separate fields with additional information such as title:

```rust
pub struct FormalPerson
{
  pub title: String,
  pub first_name: String,
  pub last_name: String,
}

impl FormalPerson
{
  pub fn new(
    title: &str,
    first_name: &str,
    last_name: &str,
  ) -> Self
  {
    Self {
      title: title.to_string(),
      first_name: first_name.to_string(),
      last_name: last_name.to_string(),
    }
  }
}
```

To greet a `FormalPerson`, we may want to greet the person with their full name and title. We could write a separate `greet_formal` function as follows:

```rust
fn greet_formal(person: &FormalPerson) -> String
{
  format!(
    "Hello, {} {} {}!",
    person.title, person.first_name, person.last_name
  )
}
```

But now we would have duplicate logic in both `greet` and `greet_formal`. In both cases we start the greeting by the word "Hello", followed by the person's full name. Our greet logic is quite simple because this is just for demo purpose. However we can imagine that in practice the logic may be much more complicated, with multiple places in the code that requires the use of other fields in the struct.

Even if we manage to deduplicate some common logic, things would still get complicated when we implement more structs that need to be handled by our greet function. For example, we would add an `Anonymous` struct to greet strangers:

```rust
pub struct Anonymous
{
  pub id: u64,
}

impl Anonymous
{
  pub fn new(id: u64) -> Self
  {
    Self { id }
  }
}

fn greet_anonymous(person: &Anonymous) -> String
{
  format!("Hello, Anonymous #{}!", person.id)
}
```

As our greet application grows more complex, we want to make use of more advanced features in Rust to help us design proper abstractions for our greet function.

## The `HasName` Trait

In the different versions of greet functions that we defined earlier, they all greet a person with "Hello", followed by the person's name derived from available information. In Rust we can define a _trait_ called `HasName` that abstracts the logic of getting a person's name:

```rust
pub trait HasName
{
  fn name(&self) -> String;
}
```

For readers coming from OOP background, Rust traits may look similar to OOP concepts such as interfaces or classes. While it is true that we can use Rust traits by treating them like their OOP counterparts, there are also many powerful concepts that only Rust traits can provide. But first let's take a look at how we can use `HasName` in an OOP fashion.

First, we can implement `HasName` for `CasualPerson` as follows:

```rust
impl HasName for CasualPerson
{
  fn name(&self) -> String
  {
    self.name.clone()
  }
}
```

In Rust, traits can be implemented for a given type, with the type `Self` being a type alias to the type that implements the trait. The trait method `name` accepts a special argument `&self`, which has the type `&Self` and returns a `String`. For the case of `CasualPerson`, we can simply get the person's name from its `name` field (which is different from the `name` method). We also use the `clone` method from the `Clone` trait to make a copy of the name string and return it.

Now that we have implemented `HasName` for `CasualPerson`, we can use the method call notation `.name()` to get the name of a person:

```rust
let person = CasualPerson::new("Alice");
let name = person.name();
```

Behind the scene, Rust recognizes the trait methods with `self` in the first argument, and desugars them into a fully qualified function call with the self value being the first argument. So the call to `person.name()` is equivalent to follows:

```rust
let name = HasName::name(&person);
```

In many cases, we use `self` in trait methods as a _syntactic sugar_ so that trait methods can be called in a more convenient way. It is important to understand that unlike in OOP, we are not obligated to always use `self` in trait methods. For example, it is totally fine if we redefine `HasName` as follows:

```rust
pub trait HasName2
{
  fn name(person: &Self) -> String;
}
```

The main difference of defining `HasName2` this way is that we now have to always use the fully qualified syntax `HasName2::name(person)` to get the name of a person. While it may look inconvenient, this is a powerful feature to keep in mind for the future, as we can define advanced trait methods that do not use the `Self` type directly in the first argument.

Next, we also define the `HasName` implementation for `FormalPerson` and `Anonymous`:

```rust
impl HasName for FormalPerson
{
  fn name(&self) -> String
  {
    format!("{} {} {}", self.title, self.first_name, self.last_name)
  }
}

impl HasName for Anonymous
{
  fn name(&self) -> String
  {
    format!("Anonymous #{}", self.id)
  }
}
```

With the trait implementations in place, we will then look at how we can define greet functions that can work with _any_ type that implements `HasName`.

## Type Erasure with `dyn` Traits

For programmers coming from OOP background, Rust offers the `dyn` trait feature that _erases_ information about the original type and make it work similar to objects in OOP. Using `dyn`, we can define our greet function to accept references to values that implement `HasName`, without knowing the type of the value:

```rust
fn greet(person: &dyn HasName) -> String
{
  format!("Hello, {}!", person.name())
}
```

The way the above `greet` function works is that during runtime, there is a single version of the `greet` function that can work with all values whose type implement `HasName`. When the function is called, it is given a pointer that points to a value together with the _virtual table_ that contain the address of the trait methods. The function can then use the virtual table to perform _dynamic dispatch_ to call the actual trait method that is implemented for the given value.

An advantage of using `dyn` is that since there is only one version of `greet` being compiled, the resulting binary takes less space as compared to the _generic_ approach that we will look into later. As a trade off, there is additional overhead of using dynamic dispatch to look up the method calls, and the compiler may not be able to optimize the function by inlining the implementation for a specific type.

## Existential Types and `dyn` Trait Objects

When we convert a value into `dyn` trait object, we are effectively erasing the original type for the value and turn it into a value with an _existential type_. This means that when we see a `dyn` trait object, we only know that there _exist_ some type for the underlying value, but there is no way to recover what type it is. The most common way we can erase the type of a value is by putting it into a `Box`:

```rust
let person1: Box<dyn HasName> = Box::new(CasualPerson::new("Alice"));
```

Once we have created a `Box<dyn HasName>` value, we lost the information that the original type behind the boxed value is in fact `CasualPerson`. Because of this, we can create `Box<dyn HasName>` constructed from other types such as `FormalPerson`, and Rust would treat them as having the same type. With that we can for instance put the different boxed values into the same `Vec`:

```rust
let person2: Box<dyn HasName> = Box::new(
  FormalPerson::new("Mr.", "John", "Smith"));
let persons: Vec<Box<dyn HasName>> = vec![person1, person2];
```

Using `dyn` trait objects, it is relatively straightforward to write a function like `greet_many`, which accepts a `Vec<Box<dyn HasName>>` and return a `Vec<String>` containing greetings for all persons inside the given list:

```rust
fn greet_many(persons: Vec<Box<dyn HasName>>) -> Vec<String>
{
  persons
    .iter()
    .map(|person| greet(person.as_ref()))
    .collect()
}
```

The main limitation however is that we cannot give it other versions of the list, such as `Vec<CasualPerson>` or `Vec<FormalPerson>`. The values must first be converted into boxed values, and doing so may have additional performance overhead. We will later look at how the generic version of `greet_many` can make the implementation more concise and also more performant.

## Limitations of `dyn` Trait Objects

It is most useful to use `dyn` trait objects when we need data structures to hold values from multiple concret types at the same type. However the `dyn` trait objects in Rust do not have the full capabilities of existential types in languages such as OCaml and Haskell. There are also various [object safety restrictions](https://doc.rust-lang.org/reference/items/traits.html#object-safety) of how a `dyn` trait can be defined. Most infamously, the `Self` type in trait object cannot appear in places other than the first position of a trait method.

```rust
trait Clone {
  fn clone(&self) -> Self;
}
```

As a result, traits such as `Clone` shown above cannot be made into a `dyn` trait object. This makes sense because consider the following use:

```rust
let boxed: Box<dyn Clone> = Box::new("Hello".to_string());
let cloned: ?? = boxed.clone();
```

If `Clone` were allowed to be made into a `dyn` trait object, then what should be the type of `cloned`? Since the original type has been erased, there is no way Rust can know what the return type `Self` should be. While it is trivial to determine that `Self` should be `String` in the above example, in general it is not possible for us to determine that in arbitrary code.

## Generic Programming

The use of object-safe trait definition is only a small subset of features that Rust traits can provide. More generally, traits are much more useful when used in conjunction with _generic programming_. At the most basic level, generic programming in Rust is about writing an abstract version of a function that can be _specialized_ to work with multiple concrete types.

We can see how generic programming work in action by seeing a generic version of the `greet` function:

```rust
fn greet<P: HasName>(person: &P) -> String
{
  format!("Hello, {}!", person.name())
}
```

The above version of `greet` function can work against _any_ type `P` that implements `HasName`. As a result, we can for example call `greet` with both `CasualPerson` and `FormalPerson`:

```rust
let person1 = CasualPerson::new("Alice");
let person2 = FormalPerson::new("Mr.", "John", "Smith");

greet(&person1);
greet(&person2);
```

During compilation, Rust would generate _two_ different versions of the `greet` function, and call the specialized version of the function depending on the type. So the above code would be roughly equivalent to writing the following non-generic version of the code:

```rust
fn greet_casual(person: &CasualPerson) -> String
{
  format!("Hello, {}!", CasualPerson::name(&person))
}

fn greet_formal(person: &FormalPerson) -> String
{
  format!("Hello, {}!", FormalPerson::name(&person))
}

let person1 = CasualPerson::new("Alice");
let person2 = FormalPerson::new("Mr.", "John", "Smith");

greet_casual(&person1);
greet_formal(&person2);
```

The process of generating multiple specialized versions of a generic function is called _monomorphization_. Since the Rust compiler can see inside the body of each specialized function, it can also perform additional optimization such as inlining to speed up the code.

## Dictionary Passing Style

It may look a little magical of how Rust actually process and monomorphize generic functions. One way to understand this is by thinking of traits as _dictionary_ structs that contains functions that implement the methods for the traits. For the case of the `HasName` trait, we can imagine it being a `HasNameDict` struct:

```rust
pub struct HasNameDict<P>
{
  pub name: fn(&P) -> String,
}
```

In the dictionary version of the definition, instead of having an implicit `Self` type, the struct is now explicitly parameterized by the type parameter `P`. The struct contains a field `name`, which is a function pointer toward the implementation that accepts a `&P` and returns a `String`.

A dictionary version of the generic greet function can be defined without the `P: HasName` trait bound, and instead works with any type `P` provided the caller provides a `HasNameDict<P>` struct that contains the imptementation:

```rust
fn greet_with_dict<P>(
  dict: &HasNameDict<P>,
  person: &P,
) -> String
{
  format!("Hello, {}!", (dict.name)(person))
}
```

Now instead of calling `HasName::name` directly, the `greet_with_dict` function would use the function field in `dict.name` to call the method. To call this function, we would have to manually construct the dictionary such as follows:

```rust
fn formal_person_name(person: &FormalPerson) -> String {
  format!("{} {} {}",
    person.title, person.first_name, person.last_name)
}

let formal_person_has_name_dict = HasNameDict {
  name: formal_person_name
};

let person = FormalPerson::new("Mr.", "John", "Smith");
greet_with_dict(&formal_person_has_name_dict, &person);
```

As we can see, the dictionary version of `HasName` require much more boilerplate as compared to the trait definition. Fortunately, Rust does all of this for us at compile time, so we don't need to worry about constructing and passing the dictionaries manually. In fact, the dictionaries are not present at runtime, so there is also no overhead of calling the dictionary methods.

Nevertheless, dictionary passing style serves as a good mental model for us to picture how traits work in Rust. This is in particular important as it will help us learn about more advanced trait implementation approaches in later sections.

(to be continue..)
