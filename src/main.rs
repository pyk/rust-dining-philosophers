use std::thread;
use std::sync::{Mutex, Arc};

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher{
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep_ms(150);
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);
        thread::sleep_ms(1000);
        println!("{} is done eating.", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),    
        Mutex::new(()),    
        Mutex::new(()),    
        Mutex::new(()),    
        Mutex::new(()),    
    ]});

    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Max", 2, 3),
        Philosopher::new("Emma Goldman", 3, 4),
        Philosopher::new("Michel Foucault", 0, 4),
    ];

    println!("Name of Philosophers:");
    /* Take reference instead of variable binding.
     * for loop scope it's borrowing the philosophers
     * instead of owning it. it can't change the
     * philoshopers though. */
    // for p in &philosophers {
        // println!("{}", p.name);
    // }

    let handlers: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();

        return thread::spawn(move || {
            p.eat(&table);
        });
    }).collect();

    println!("Philosophers eat the spagethi.");
    for h in handlers {
        h.join().unwrap();
    }
}
