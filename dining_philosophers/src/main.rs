/* https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/dining-philosophers.html */
use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

struct Table {
    forks: Vec<Mutex<()>>,
}

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep(Duration::from_millis(150));
        let _right = table.forks[self.right].lock().unwrap();

        println!("{}は食事をしています。", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{}は食べ終わりました。", self.name);
    }
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
        Philosopher::new("真中らぁら", 0, 1),
        Philosopher::new("南みれぃ", 1, 2),
        Philosopher::new("北条そふぃ", 2, 3),
        Philosopher::new("黒須あろま", 3, 4),
        Philosopher::new("白玉みかん", 0, 4),
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
