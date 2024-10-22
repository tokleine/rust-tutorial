fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // break and return a value for the let statement
            // return counter * 2; // ends the current function (i.e. main) and causes failure
        }
    };
    println!("The result is {result}");


    // disambiguating loops & statements
    let mut count = 0;
    'counting_up: loop { // gives the loop a 'label', always has to start with a `'`
        println!("count = {count}");
        let mut value = 10;

        'valueing_up: loop {
            println!("value = {value}");
            if value == 30 {
                break 'valueing_up;
            }
            if count == 3 {
                break 'counting_up;
            }
            value += 10;
        }

        count += 1;
    }
    println!("End count = {count}");

    // iterate over an array
    // let a = [10, 20, 30, 40, 50]; // can be reversed, see below
    let a = [10, 20, 30, 40, 50].iter().rev();

    for element in a { // neater way of iterating
        println!("the value is: {}", element);
    }


    // reverse iteration
    for counter in (1..11).rev() {
        println!("{}!", counter);
    }
    println!("LIFTOFF!!!");
}