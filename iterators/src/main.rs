struct Person {
    firts_name: String,
    last_name: String,
    occupation: String,
  }
  
  // impl IntoIterator for Person {
  //   type Item = String;
  //   type IntoIter = std::vec::IntoIter<Self::Item>;
  
  //   fn into_iter(self) -> Self::IntoIter {
  //     vec![self.firts_name, self.last_name, self.occupation].into_iter()
  //   }
  // }
  
  struct PersonIterator {
    values: Vec<String>
  }
  
  impl Iterator for PersonIterator {
    // associated type
    type Item = String;
  
    fn next(&mut self) -> Option<Self::Item> {
      if self.values.is_empty() {
        None
      } else {
        Some(self.values.remove(0))
      }
    }
  }
  
  impl IntoIterator for Person {
    type Item = String;
    type IntoIter = PersonIterator;
  
    fn into_iter(self) -> Self::IntoIter {
      PersonIterator {
        values: vec![self.firts_name, self.last_name, self.occupation]
      }
    }
  }
  
  fn main() {
    let p = Person {
      firts_name: "John".to_string(),
      last_name: "Doe".to_string(),
      occupation: "Rust Developer".to_string(),
    };
  
    // let mut i = p.into_iter();
  
    // println!("{:?}", i.next());
    // println!("{:?}", i.next());
    // println!("{:?}", i.next());
    // println!("{:?}", i.next());
  
    // for loop knows how to manage an iterator
    for val in p {
      println!("{}", val);
    }
  }
  
  // use std::collections::HashMap;
  // // for collections
  // fn main() {
  //   let mut data = HashMap::new();
  
  //   data.insert("name".to_owned(), "John");
  //   data.insert("age".to_owned(), "30");
  //   data.insert("occupation".to_owned(), "Rust Developer");
  
  
  //   // let mut data_iter = data.iter();
  //   // println!("{:?}", data_iter.next());
  
  //   for (key, val) in data {
  //     println!("{}: {}", key, val);
  //   }
  // }
  
  