#[derive(Debug, Clone)]
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn new(owner: &str, initial_balance: f64) -> Self {
    BankAccount {
            owner: owner.to_string(),
            balance: initial_balance,
        }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited ${:.2}. New balance: ${:.2}", amount, self.balance);
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount > self.balance {
            return Err("Insufficient funds".into());
        }

        self.balance -= amount;
        println!("Withdrew ${:.2}. New balance: ${:.2}", amount, self.balance);

        Ok(())
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut acc = BankAccount::new("Alice", 1000.0);

    acc.deposit(500.0);

    match acc.withdraw(200.0) {
        Ok(()) => println!("Withdrawal successful"),
        Err(msg) => println!("Error: {}", msg),
    }

    println!("Final balance: ${:.2}", acc.balance());

     let c = Circle {radius: 3.0 };
    let r = Rectangle {
        width: 4.0,
        height: 5.0,
    };

    print_area(&c);
    print_area(&r);

    println!("{}", c.describe());
    println!("{}", r.describe());

    println!("{}", c);
    println!("{}", r);

     let test_cases = vec!["42", " 101 ", "abc", "", "-5"];

    for case in test_cases {
        match parse_and_validate(case, 0, 100) {
            Ok(n) => println!("Valid: {}", n),
            Err(e) => println!("Error for {:?}: {}", case, e),
        }
    }
  }
   


  use std::fmt;

  trait Describable {
    fn describe(&self) -> String;
    // default method
    fn short_name(&self) -> String {
        format!(
            "{}",
            &self.describe()
            [..20.min(self.describe().len())]
        )
    }
  }
  
  trait Area{
    fn area(&self) -> f64;
  }
  
  #[derive(Debug)]
  struct Circle {
    radius: f64,
  }
  
  #[derive(Debug)]
  struct Rectangle{
    width: f64,
    height: f64,
  }
  
  impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
  }
  
  impl Area for Rectangle{
    fn area(&self) -> f64 {
        self.width * self.height
    }
  }
  
  impl Describable for Circle {
    fn describe(&self) -> String {
        format!("Circle with radius {:.2}", self.radius)
    }
  }
  
  // TODO 2: Implement Describable for Rectangle
  impl Describable for Rectangle {
    fn describe(&self) -> String {
        format!(
            "Rectangle with width{:.2} and height {:.2}",
            self.width, self.height
        )
    }
  }
  
  // TODO 3: Implement fmt::display for Circle
  impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.describe())
    }
  }
  
  // TODO 3: IMplement fmt::display for Rectangle
  impl fmt::Display for Rectangle{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.describe())
    }
  }
  
  // Trait object - dynamic dispatch
  fn print_area(shape: &dyn Area) {
    println!("Area = {:.4}", shape.area());
  }


 


use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    Parse(ParseIntError),
    OutOfRange {
        value: i32,
        min: i32,
        max: i32,
    },
    EmptyInput,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Parse(e) => {
                write!(f, "Parse error: {}", e)
            }

            AppError::OutOfRange { value, min, max } => {
                write!(f, "{} is not in [{}, {}]", value, min, max)
            }

            AppError::EmptyInput => {
                write!(f, "Input was empty")
            }
        }
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

fn parse_and_validate(s: &str, min: i32, max: i32) -> Result<i32, AppError> {
    if s.trim().is_empty() {
        return Err(AppError::EmptyInput);
    }

    let n: i32 = s.trim().parse()?;

    if n < min || n > max {
        return Err(AppError::OutOfRange {
            value: n,
            min,
            max,
        });
    }

    Ok(n)
}


  
  
  
