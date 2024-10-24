trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Get ready for take-off!");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Zimzalabim fly!");
    }
}

impl Human {
    fn fly(&self) {
        println!("Humans cant fly.. :(");
    }
}

fn human(p: &Human) {
    p.fly();
}

fn pilot<T: Pilot>(p: &T) {
    p.fly();
}

fn wizard<T: Wizard>(w: &T) {
    w.fly();
}

pub fn run() {
    let person = Human;
    human(&person);
    pilot(&person);
    wizard(&person);
}
