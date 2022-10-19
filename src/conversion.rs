pub fn run() {
    println!("===Conversion====");

    #[derive(Debug)]
    struct Angka {
        nilai: i32,
    }

    impl From<i32> for Angka {
        fn from(item: i32) -> Self {
            Angka { nilai: item }
        }
    }

    // dengan implementasi trait from, kita bisa membuat conversi dari i32 menjadi struct Angka
    let num = Angka::from(30);

    let num2 = Angka { nilai: 24 };
    println!("num bernilai {:?}", num);
    println!("num2 bernilai {:?}", num2);

    // cara lain conversi yaitu dengan menggunakan trait into
    let int = 26;
    let num: Angka = int.into();
    println!("num adalah {:?}", num);
}
