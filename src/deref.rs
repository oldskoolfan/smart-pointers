use std::ops::Deref;

use super::drop::CustomSmartPointer;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

pub fn run_deref() {
  let x = 5;
  let y = MyBox::new(5);

  assert_eq!(5, x);
  // smart pointer deref
  assert_eq!(5, *y);
  println!("done!");
  
  // deref coercion
  let m = MyBox::new(String::from("Rust"));
  hello(&m);

  let c = CustomSmartPointer::new("foo");
  c.say_hello();
}