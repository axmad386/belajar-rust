
    
struct Kambing {
    mandi: bool,
nama: &'static str,
}

trait Binatang {
    fn make(name: &'static str)->Self;
    fn name(&self)->&'static str;
    fn noise(&self)->&'static str;

    // trait bisa dibuat default method, 
    // jadi seperti abstract class kalau di php atau javascript
    fn talk(&self){
        println!("{} says ... {}", self.name(), self.noise());
    }
}

impl Kambing {
    fn is_clean(&self) ->bool {
        self.mandi
    }
    
    fn adus(&mut self) {
        if self.is_clean(){
            println!("{} wes adus", self.nama);
        } else {
            println!("{} lagi diadusi ...", self.name());
            self.mandi = true;
        }
    }
}

impl Binatang for Kambing {
    fn make(name:&'static str) ->Kambing {
        Kambing {nama:name, mandi: false}
    }

    fn name(&self)->&'static str {
        self.nama
    }
    fn noise(&self)->&'static str {
        if self.is_clean(){
            "mbeeekkk...."
        } else {
        "mbookkk...."
        }
    }
}

pub fn run(){
    let mut wedus: Kambing = Binatang::make("cempe");
    wedus.talk();
    wedus.adus();
    wedus.talk();
    wedus.adus();
}
