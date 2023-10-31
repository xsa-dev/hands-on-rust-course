#![allow(unused)]

fn main() {
    // let args = std::env::args().collect::<Vec<String>>();
    // println!("{args:?}");

    let n = std::env::args()
        .nth(1)
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let mut secs = 0;
    loop {
        let rest_secs = n - secs;

        println!("timer    : {secs}");
        println!("rest secs: {rest_secs}   ");
        print!("\x1b[2A");

        if rest_secs == 0 {
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(999));
        secs += 1;
    }

    println!("🔥Time is over!");
}

// timer    : 0
// rest secs: 10
//

// cargo run -q -- 10
