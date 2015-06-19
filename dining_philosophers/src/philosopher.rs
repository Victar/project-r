struct Pholisopher {
    name: String;
}

impl Philospher {

    fn  new(name: &str)-> Philospher {
        Philosopher {
	    name: name.to_string(),
	}
    }
}


fn main(){
    let p1 = Philospher::new("Baruch Spinoza");
    let p2 = Philospher::new("Gilles Deleuze");
    let p3 = Philospher::new("Karl Marx");
    let p4 = Philospher::new("Friedrich Nietzscge");
    let p5 = Philospher::new("Michel Foucault");
}
