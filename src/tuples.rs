use core::fmt;


pub fn run() {
    println!("===tuples===");
    fn reverse_pair(pair: (i32, bool)) -> (bool, i32) {
        let (a, b) = pair;
        (b, a)
    }

    let pair = (2, true);
    println!("pair is {:?}", pair);
    println!("first item of pair is {}", pair.0);
    println!("second item of pair is {}", pair.1);
    println!("the reversed pair is {:?}", reverse_pair(pair));

    #[derive(Debug)]
    struct Matrix (f32, f32, f32, f32);

    // secara default custom struct tidak bisa diprint menggunakan default fmt::Display
    // jadi kita harus mendefinisikan sendiri cara rendernya
    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{} {}]\n[{} {}]", self.0, self.1, self.2, self.3)
        }
    }

    let matrix = Matrix(1.1, 1.2, 1.3, 1.4);
    println!("{:?}", matrix); // ini akan diprint secara default menggunakan fmt::Debug
    println!("{}", matrix); // ini akan dirender sesuai yang kita declare di atas oleh fmt::Display
}
