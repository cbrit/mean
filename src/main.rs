use std::io;

// For learnig to use vectors and reviewing match statements.
fn main() {
    println!("Enter as many values as you like, each on their own line.");
    println!("When you are finished, enter a blank line  or anything that isn't an integer to see the mean of the values.");

    let mut v = Vec::<f32>::new();

    loop {
        let mut entry = String::new();

        match io::stdin()
            .read_line(&mut entry) {
                Ok(_) => {
                    match entry.trim().parse() {
                        Ok(num) => &v.push(num),
                        Err(_) => break,
                    };
                },
                Err(_) => continue,
            };
    }

    let mut total: f32 = 0.0;

    for num in &v {
        total = total + num;
    }

    let mean = total / v.len() as f32;

    println!("The mean is {}!", mean);
}
