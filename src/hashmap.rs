use std::collections::HashMap;

pub fn run() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Team A"), 10);

    // mengambil nilai dari HashMap dan insert jika belum ada
    scores.entry(String::from("Team A")).or_insert(11);
    scores.entry(String::from("Team B")).or_insert(12);
    println!("{:?}", scores);
    println!("{:?}", scores.get(&String::from("Team A")));

    // mari kita hitung berapa kali kata muncul di kalimat ini
    let text = "Hari ini saya laper dan pingin makan karena memang laper banget";
    let mut result = HashMap::new();

    for t in text.split_whitespace() {
        let count = result.entry(t).or_insert(0);
        *count += 1;
    }
    println!("{:?}", result);
}
