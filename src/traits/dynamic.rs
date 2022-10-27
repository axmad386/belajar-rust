struct Sheep {}
struct Cow {}

trait Animal {
  // ini semacam interface kalau di php
  fn noise(&self) -> &'static str;
}

// implementasi Animal trait untuk Sheep
impl Animal for Sheep {
  fn noise(&self) -> &'static str {
      "mbeeeeek!"
  }
}

impl Animal for Cow {
  fn noise(&self) -> &'static str {
      "Mooouuuuu!"
  }
}

// create random animal
// semua function di rust harus ada return type, karena animal yang random kita bisa pakai Box
// agar tetap valid saat compile time
// dyn artinya dinamis, rust tidak tahu Animal tipe apa saat compile time
fn random_animal(random_number: f64) -> Box<dyn Animal> {
  if random_number < 0.5 {
    Box::new(Sheep {})
  } else {
    Box::new(Cow {})
  }
}

pub fn run() {
    println!("====Dyn Trait=====");

    let random_number = 0.246;
    let animal = random_animal(random_number);
    println!("The chosen animal says {}", animal.noise());
}
