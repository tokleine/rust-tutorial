fn main() {
    let mut comp_1: usize = 0;
    let mut comp_2: usize= 1;

    let mut fib_sum: usize = 1;
    while fib_sum < usize::MAX {
        fib_sum = comp_1.saturating_add(comp_2);
        comp_1 = comp_2;
        comp_2 = fib_sum;
        println!("Fibonacci sum: {}", fib_sum);
    }

    println!("Max possible reached (usize, {}): {}", usize::MAX, fib_sum);


}
