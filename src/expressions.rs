pub fn run() {
    println!("===Expressions===");
    // expression diakhiri dengan semicolon ;
    let x = 5u32;

    let y = {
        // yang di dialam kurung kurawal ini juga expression
        let x_squared = x * x;
        // jika expression terakhir di dalam kurung kurawal tidak diakhiri dengan semicolon,
        // maka akan menjadi return nilai
        x_squared + x // ini akan menjadi nilai y
                      // jika diakhiri dengan semicolon maka nilai akhir menjadi ()
    };
    println!("x is {}", x);
    println!("y is {:?}", y);
}
