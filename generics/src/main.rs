///Let's look at a largest function that finds the largest
/// i32 integer in a list slice.
/*fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest{
            largest = item;
        }
    }
    largest
}
///Returns the largest char
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest{
            largest = item;
        }
    }
    largest
}

fn main() {
    let integer_list = [1, 2, 3, 4, 5, 6, 7];
    let result = largest_i32 (&integer_list);
    println!("{}", result);

    let char_list = ['u', 't', 'f', 'k'];
    let result = largest_char(&char_list);
    println!("{}", result);
}
*/
/// Let's rewrite the above code to remove 
/// duplication by using Generics
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    let num_list = vec![1, 2, 3, 4, 5];
    let result = largest (&num_list);
    println!("The largest number is {}", result);

    let char_list = ['u', 't', 'f', 'k'];
    let result = largest(&char_list);
    println!("The largest character is {}", result);
}