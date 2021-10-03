#![allow(dead_code)]

mod v1
{
  fn greet(name: &str) -> String
  {
    format!("Hello, {}!", name)
  }
}

mod v2
{
  pub struct CasualPerson
  {
    pub name: String,
  }

  impl CasualPerson
  {
    pub fn new(name: &str) -> Self
    {
      Self {
        name: name.to_string(),
      }
    }
  }

  fn greet(person: &CasualPerson) -> String
  {
    format!("Hello, {}!", person.name)
  }
}

mod v3
{
  pub use crate::v2::CasualPerson;

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

  fn greet_formal(person: &FormalPerson) -> String
  {
    format!(
      "Hello, {} {} {}!",
      person.title, person.first_name, person.last_name
    )
  }

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

  pub trait HasName
  {
    fn name(&self) -> String;
  }

  impl HasName for FormalPerson
  {
    fn name(&self) -> String
    {
      format!("{} {} {}", self.title, self.first_name, self.last_name)
    }
  }

  impl HasName for CasualPerson
  {
    fn name(&self) -> String
    {
      self.name.clone()
    }
  }

  impl HasName for Anonymous
  {
    fn name(&self) -> String
    {
      format!("Anonymous #{}", self.id)
    }
  }

  fn greet_dyn(person: &dyn HasName) -> String
  {
    format!("Hello, {}!", person.name())
  }

  fn greet_many_dyn(persons: Vec<Box<dyn HasName>>) -> Vec<String>
  {
    persons
      .iter()
      .map(|person| greet_dyn(person.as_ref()))
      .collect()
  }

  fn greet_generic<Person: HasName>(person: &Person) -> String
  {
    format!("Hello, {}!", person.name())
  }

  pub struct HasNameDict<Person>
  {
    pub name: fn(&Person) -> String,
  }

  fn greet_with_dict<Person>(
    dict: HasNameDict<Person>,
    person: &Person,
  ) -> String
  {
    format!("Hello, {}!", (dict.name)(person))
  }

  fn greet_impl(person: &impl HasName) -> String
  {
    format!("Hello, {}!", person.name())
  }

  pub fn greet_many_generic<Person: HasName>(
    persons: &Vec<Person>
  ) -> Vec<String>
  {
    persons.iter().map(greet_generic).collect()
  }
}

mod v4
{
  use crate::v3::{
    Anonymous,
    CasualPerson,
    FormalPerson,
    HasName,
  };

  enum AnyPerson
  {
    Formal(FormalPerson),
    Casual(CasualPerson),
    Anon(Anonymous),
  }

  impl HasName for AnyPerson
  {
    fn name(&self) -> String
    {
      match self {
        Self::Formal(person) => person.name(),
        Self::Casual(person) => person.name(),
        Self::Anon(person) => person.name(),
      }
    }
  }

  #[test]
  fn test()
  {
    use crate::v3::greet_many_generic;

    let persons = vec![
      AnyPerson::Formal(FormalPerson::new("Mr.", "John", "Smith")),
      AnyPerson::Casual(CasualPerson::new("Alice")),
      AnyPerson::Anon(Anonymous::new(8)),
    ];

    assert_eq!(
      greet_many_generic(&persons),
      vec![
        "Hello, Mr. John Smith!",
        "Hello, Alice!",
        "Hello, Anonymous #8!",
      ]
    );
  }
}

mod v5
{
  use crate::v3::{
    Anonymous,
    CasualPerson,
    FormalPerson,
    HasName,
  };

  pub enum Either<A, B>
  {
    Left(A),
    Right(B),
  }

  impl<A: HasName, B: HasName> HasName for Either<A, B>
  {
    fn name(&self) -> String
    {
      match self {
        Self::Left(person) => person.name(),
        Self::Right(person) => person.name(),
      }
    }
  }

  pub type AnyPersonGeneric =
    Either<FormalPerson, Either<CasualPerson, Anonymous>>;

  pub struct AnyPerson(
    pub Either<FormalPerson, Either<CasualPerson, Anonymous>>,
  );

  impl HasName for AnyPerson
  {
    fn name(&self) -> String
    {
      self.0.name()
    }
  }

  impl AnyPerson
  {
    pub fn formal(person: FormalPerson) -> Self
    {
      Self(Either::Left(person))
    }

    pub fn casual(person: CasualPerson) -> Self
    {
      Self(Either::Right(Either::Left(person)))
    }

    pub fn anon(person: Anonymous) -> Self
    {
      Self(Either::Right(Either::Right(person)))
    }
  }

  pub fn make_persons() -> Vec<AnyPerson>
  {
    vec![
      AnyPerson::formal(FormalPerson::new("Mr.", "John", "Smith")),
      AnyPerson::casual(CasualPerson::new("Alice")),
      AnyPerson::anon(Anonymous::new(8)),
    ]
  }

  #[test]
  fn test()
  {
    use crate::v3::greet_many_generic;

    let persons = make_persons();

    assert_eq!(
      greet_many_generic(&persons),
      vec![
        "Hello, Mr. John Smith!",
        "Hello, Alice!",
        "Hello, Anonymous #8!",
      ]
    );
  }
}

mod v6
{
  use crate::v3::HasName;

  pub trait Greeter
  {
    fn greet(
      &self,
      person: &impl HasName,
    ) -> String;
  }

  struct HelloGreeter;

  impl Greeter for HelloGreeter
  {
    fn greet(
      &self,
      person: &impl HasName,
    ) -> String
    {
      format!("hello, {}!", person.name())
    }
  }

  pub struct WordGreeter
  {
    pub greet_word: String,
  }

  impl Greeter for WordGreeter
  {
    fn greet(
      &self,
      person: &impl HasName,
    ) -> String
    {
      format!("{}, {}!", self.greet_word, person.name())
    }
  }

  impl WordGreeter
  {
    pub fn new(greet_word: &str) -> Self
    {
      Self {
        greet_word: greet_word.to_string(),
      }
    }
  }

  pub fn greet_many<Greet: Greeter, Person: HasName>(
    greeter: &Greet,
    persons: &Vec<Person>,
  ) -> Vec<String>
  {
    persons.iter().map(|person| greeter.greet(person)).collect()
  }

  #[test]
  fn test()
  {
    use crate::v5::make_persons;

    let persons = make_persons();

    greet_many(&HelloGreeter, &persons);

    let greeter = WordGreeter::new("Welcome");

    assert_eq!(
      greet_many(&greeter, &persons),
      vec![
        "Welcome, Mr. John Smith!",
        "Welcome, Alice!",
        "Welcome, Anonymous #8!",
      ]
    );
  }
}

mod v7
{
  use crate::{
    v3::{
      Anonymous,
      CasualPerson,
      FormalPerson,
    },
    v5::{
      AnyPerson,
      AnyPersonGeneric,
      Either,
    },
  };

  pub trait Greeter<Person>
  {
    fn greet(
      &self,
      person: &Person,
    ) -> String;
  }

  fn greet_many<P, G: Greeter<P>>(
    greeter: &G,
    persons: &Vec<P>,
  ) -> Vec<String>
  {
    persons.iter().map(|person| greeter.greet(person)).collect()
  }

  impl<G, A, B> Greeter<Either<A, B>> for G
  where
    G: Greeter<A>,
    G: Greeter<B>,
  {
    fn greet(
      &self,
      person: &Either<A, B>,
    ) -> String
    {
      match person {
        Either::Left(person) => self.greet(person),
        Either::Right(person) => self.greet(person),
      }
    }
  }

  struct CustomGreeter;

  impl Greeter<FormalPerson> for CustomGreeter
  {
    fn greet(
      &self,
      person: &FormalPerson,
    ) -> String
    {
      format!("Welcome back, {} {}!", person.title, person.last_name)
    }
  }

  impl Greeter<CasualPerson> for CustomGreeter
  {
    fn greet(
      &self,
      person: &CasualPerson,
    ) -> String
    {
      format!("Hello, {}!", person.name)
    }
  }

  impl Greeter<Anonymous> for CustomGreeter
  {
    fn greet(
      &self,
      person: &Anonymous,
    ) -> String
    {
      format!("Hello stranger, your ID is {}.", person.id)
    }
  }

  struct AnyPersonGreeter<G>(G);

  impl<G> Greeter<AnyPerson> for AnyPersonGreeter<G>
  where
    G: Greeter<AnyPersonGeneric>,
  {
    fn greet(
      &self,
      person: &AnyPerson,
    ) -> String
    {
      self.0.greet(&person.0)
    }
  }

  #[test]
  fn test()
  {
    use crate::v5::make_persons;

    let persons = make_persons();

    assert_eq!(
      greet_many(&AnyPersonGreeter(CustomGreeter), &persons),
      vec![
        "Welcome back, Mr. Smith!",
        "Hello, Alice!",
        "Hello stranger, your ID is 8."
      ]
    );
  }
}

mod v8
{
  use crate::{
    v3::{
      Anonymous,
      CasualPerson,
      FormalPerson,
      HasName,
    },
    v5::{
      AnyPerson,
      Either,
    },
    v6::WordGreeter,
  };

  pub trait Greeter<Person>
  {
    fn greet(
      &self,
      person: &Person,
    ) -> String;
  }

  pub struct Unit<G>(G);

  impl<G, A, B> Greeter<Either<A, B>> for Unit<G>
  where
    Unit<G>: Greeter<A>,
    Unit<G>: Greeter<B>,
  {
    fn greet(
      &self,
      person: &Either<A, B>,
    ) -> String
    {
      match person {
        Either::Left(person) => self.greet(person),
        Either::Right(person) => self.greet(person),
      }
    }
  }

  impl<G1, G2, P> Greeter<P> for Either<G1, G2>
  where
    G1: Greeter<P>,
    G2: Greeter<P>,
  {
    fn greet(
      &self,
      person: &P,
    ) -> String
    {
      match self {
        Either::Left(g) => g.greet(person),
        Either::Right(g) => g.greet(person),
      }
    }
  }

  pub trait NameGreeter
  {
    fn greet_name(
      &self,
      person: &impl HasName,
    ) -> String;
  }

  pub struct WithName<G>(G);

  impl<G: NameGreeter, P: HasName> Greeter<P> for WithName<G>
  {
    fn greet(
      &self,
      person: &P,
    ) -> String
    {
      self.0.greet_name(person)
    }
  }

  impl NameGreeter for WordGreeter
  {
    fn greet_name(
      &self,
      person: &impl HasName,
    ) -> String
    {
      format!("{}, {}!", self.greet_word, person.name())
    }
  }

  pub struct PoliteGreeter;

  impl Greeter<FormalPerson> for Unit<PoliteGreeter>
  {
    fn greet(
      &self,
      person: &FormalPerson,
    ) -> String
    {
      format!("Welcome back, {} {}!", person.title, person.last_name)
    }
  }

  impl Greeter<CasualPerson> for Unit<PoliteGreeter>
  {
    fn greet(
      &self,
      person: &CasualPerson,
    ) -> String
    {
      format!("Hello, {}!", person.name)
    }
  }

  impl Greeter<Anonymous> for Unit<PoliteGreeter>
  {
    fn greet(
      &self,
      person: &Anonymous,
    ) -> String
    {
      format!("Hello stranger, your ID is {}.", person.id)
    }
  }

  pub struct PersonGreeter<P>(P);

  impl<P: HasName> Greeter<FormalPerson> for Unit<PersonGreeter<P>>
  {
    fn greet(
      &self,
      person: &FormalPerson,
    ) -> String
    {
      format!(
        "Greetings, {} {}! My name is {}",
        person.title,
        person.last_name,
        self.0 .0.name()
      )
    }
  }

  impl<P: HasName> Greeter<CasualPerson> for Unit<PersonGreeter<P>>
  {
    fn greet(
      &self,
      person: &CasualPerson,
    ) -> String
    {
      format!("Hi, {}! I am {}", person.name, self.0 .0.name())
    }
  }

  impl<P: HasName> Greeter<Anonymous> for Unit<PersonGreeter<P>>
  {
    fn greet(
      &self,
      person: &Anonymous,
    ) -> String
    {
      format!("Hello, stranger with ID #{}! What is your name?", person.id)
    }
  }

  fn greet_many<P, G: Greeter<P>>(
    greeters: &Vec<G>,
    persons: &Vec<P>,
  ) -> Vec<String>
  {
    greeters
      .iter()
      .map(|greeter| {
        persons
          .iter()
          .map(|person| greeter.greet(person))
          .collect::<Vec<_>>()
      })
      .flatten()
      .collect()
  }

  pub type AnyGreeterGeneric = Either<
    Unit<PoliteGreeter>,
    Either<Unit<PersonGreeter<AnyPerson>>, WithName<WordGreeter>>,
  >;

  pub struct AnyGreeter(pub AnyGreeterGeneric);

  impl Greeter<AnyPerson> for AnyGreeter
  {
    fn greet(
      &self,
      person: &AnyPerson,
    ) -> String
    {
      self.0.greet(&person.0)
    }
  }

  impl AnyGreeter
  {
    pub fn polite(greeter: PoliteGreeter) -> Self
    {
      Self(Either::Left(Unit(greeter)))
    }

    pub fn person(greeter: AnyPerson) -> Self
    {
      Self(Either::Right(Either::Left(Unit(PersonGreeter(greeter)))))
    }

    pub fn word(greeter: WordGreeter) -> Self
    {
      Self(Either::Right(Either::Right(WithName(greeter))))
    }
  }

  #[test]
  fn test()
  {
    use crate::v5::{
      make_persons,
      AnyPerson,
    };

    let greeters: Vec<AnyGreeter> = vec![
      AnyGreeter::polite(PoliteGreeter),
      AnyGreeter::person(AnyPerson::casual(CasualPerson::new("Bob"))),
      AnyGreeter::word(WordGreeter::new("Hi")),
    ];

    let persons = make_persons();

    assert_eq!(
      greet_many(&greeters, &persons),
      vec![
        "Welcome back, Mr. Smith!",
        "Hello, Alice!",
        "Hello stranger, your ID is 8.",
        "Greetings, Mr. Smith! My name is Bob",
        "Hi, Alice! I am Bob",
        "Hello, stranger with ID #8! What is your name?",
        "Hi, Mr. John Smith!",
        "Hi, Alice!",
        "Hi, Anonymous #8!"
      ]
    );
  }
}
