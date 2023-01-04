# BELAJAR RUST

## Tujuan
Repo ini dibuat untuk kalangan sendiri. Tujuan repo ini adalah untuk belajar rust lebih mudah menggunakan CLI.
Sumber materi diambil dari https://doc.rust-lang.org/stable/rust-by-example/

## Instalasi
Lihat petunjuk instalasi rust di https://www.rust-lang.org/tools/install
## Cara Penggunaan
Setiap materi dibuat module. Lihat folder `/src`, di dalamnya terdapat banyak module misalkan `conversion, enum, expression` dan lain-lain.
Setiap module wajib berisi method `run`, di mana method ini yang akan kita panggil saat testing kode yang kita pelajari.
Semua module diregistrasikan ke dalam satu file yaitu `main.rs` sehingga dapat dijalankan melalui cli.

## Cara Menjalankan Kode Modul
Sebagai contoh kita ingin menjalankan kode di module `conversion`, maka jalankan perintah berikut:
```
cargo run conversion
```
Modul dapat berisi sub modul, misal modul `traits` berisi submodul `basic, derive, dynami` dan lain-lain. Untuk menjalankan kode di sub modul jalankan perintah berikut:
```
cargo run [module] [submodule]
```
contoh: `cargo run traits basic`
