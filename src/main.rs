struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        return Philosopher{
            name: name.to_string(),
        };
    }

    fn eat(&self) {
        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let philosophers = vec![
        Philosopher::new("Judith Butler"),
        Philosopher::new("Gilles Deleuze"),
        Philosopher::new("Karl Max"),
        Philosopher::new("Emma Goldman"),
        Philosopher::new("Michel Foucault"),
    ];

    println!("Name of Philosophers:");
    /* Take reference instead of variable binding.
     * for loop scope it's borrowing the philosophers
     * instead of owning it. it can't change the
     * philoshopers though. */
    for p in &philosophers {
        println!("{}", p.name);
    }

    println!("Philosophers eat the spagethi.");
    for p in &philosophers {
        p.eat();
    }
}
