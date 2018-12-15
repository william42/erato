use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut heap = BinaryHeap::new();
    for n in 2_u64.. {
        if let Some(&Reverse((m, p))) = heap.peek() {
            if m == n {
                let mut k = n;
                print!("{} = ", n);
                while let Some(&Reverse((m, p))) = heap.peek() {
                    if m > n {
                        break;
                    }
                    heap.pop();
                    heap.push(Reverse((n+p, p)));
                    while k % p == 0 {
                        k = k / p;
                        print!("{}", p);
                        if k > 1 {
                            print!(" * ");
                        }
                    }
                }
                if k > 1 {
                    print!("{}", k);
                }
                println!("");
            } else {
                heap.push(Reverse((n * n, n)));
                println!("{} prime", n);
            }
        } else { // n = 2
            heap.push(Reverse((4, 2)));
            println!("2 prime");
        }
        sleep(Duration::new(1, 0));
    }
}
