pub struct CustomSmartPointer<'a> {
  data: &'a str,
}

impl<'a> CustomSmartPointer<'a> {
  pub fn new(val: &'a str) -> Self {
    Self {
      data: val,
    }
  }

  pub fn say_hello(&self) {
    println!("Saying hello from {}", self.data);
  }
}

impl Drop for CustomSmartPointer<'_> {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data `{}`!", self.data);
  }
}

pub fn run_drop() {
  let c = CustomSmartPointer {
    data: "my stuff: c",
  };
  let d = CustomSmartPointer {
    data: "other stuff: d",
  };
  println!("CustomSmartPointers created");
  d.say_hello();
  drop(c);
  println!("CustomSmartPointer c dropped before end of main");
}
