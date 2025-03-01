
// Fn : Immutable borrow
// FnMut : Mutable borrow
// FnOnce : takes ownership of the variables.

// Every closure implements FnOnce trait, because every closure should be called atleast once.

// T has a trait bound of Fn trait, which means T should be a closure.
struct Credentials<T> where T: Fn(&str, &str) -> bool {
    username: String,
    password: String,
    validator: T,
  }
  
  impl<T> Credentials<T> where T: Fn(&str, &str) -> bool {
    fn is_valid(&self) -> bool {
      // To call closure stored inside struct, wrap it(closure) in parenthesis.
      (self.validator)(&self.username, &self.password)
    }
  } 
  
  // fn validate_credentials(username: &str, password: &str) -> bool {
  //   !username.is_empty() && !password.is_empty()
  // }
  
  fn main() {
     let weak_password = "password123!".to_owned();
     
     let validator = |username: &str, password: &str| {
      !username.is_empty() &&
      !password.is_empty() &&
      password.len() > 8 &&
      // can pass array of characters to contains method.
      password.contains(['!', '@', '#', '$', '%', '^', '&', '*']) &&
      // will store the weak_password value in the closure, which can be accessed even in other scope.
      password != weak_password
    };
  
    let creds = Credentials {
      username: "admin".to_string(),
      password: "password@123".to_string(),
      validator
    };
  
    println!("Credentials are valid: {}", creds.is_valid());
  
    let password_validator = get_password_validator(8, true);
  
    let default_creds = get_defult_creds(password_validator);
  
    println!("Credentials are valid: {}", default_creds.is_valid());
  
  }
  
  // struct having closures as a return type which has trait bounds.
  fn get_defult_creds<T>(f: T) -> Credentials<T> where T: Fn(&str, &str) -> bool {
    Credentials {
      username: "guest".to_string(),
      password: "password123!".to_string(),
      validator: f
    }
  }
  
  // function that return closure.
  // Box<dyn Fn(&str, &str) -> bool> is a dynamic type which implements "Fn(&str, &str) -> bool"
  // is we are using trait bound as return type = Trait Object, no need to write dynamic box pointers. 
  // triat object is used as we don't know the size of closure at compile time as variables capturing by the closure may have different sizes.
  fn get_password_validator(min_len: usize, special_char: bool) -> Box<dyn Fn(&str, &str) -> bool> {
    // need to borrow min_len here, else it will be dropped.
    // use move keyword to borrow.
    if special_char {
      Box::new(move |_: &str, password: &str| {
        password.len() >= min_len &&
        password.contains(['!', '@', '#', '$', '%', '^', '&', '*'])
      })
    } else {
      Box::new(move |_: &str, password: &str| password.len() >= min_len)
    }
  }