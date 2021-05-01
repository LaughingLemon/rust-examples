
fn main() {
    let mut expensive_closure = Cacher::new(
        |num| {
            println!("being called with {}", num);
            num
        }
    );

    println!("Expensive closure is {}", expensive_closure.value(5));
    println!("Expensive closure is {}", expensive_closure.value(5));
    println!("Expensive closure is {}", expensive_closure.value(7));
    println!("Expensive closure is {}", expensive_closure.value(5));
}

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}