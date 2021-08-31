// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn coordinates() -> (i32, i32) {
    (1, 7)
}

fn main() {
    let (_x, y) = coordinates();

    if y > 5 {
        println!("Greater than 5")
    } else if y > 5 {
        println!("Less than 5")
    } else {
        println!("equal to 5")
    }
}
