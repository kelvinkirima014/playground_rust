///This code finds the largest number in a particular list.
/* 
fn main() {
    let num_list = vec![1, 2, 3, 4, 5];
    let mut largest = num_list[0];

    for num in num_list  {
        if num > largest{
            largest = num;
        }
    }
    println!("The largest number is {}", largest);

    let num_list = vec![1, 2, 3, 12, 5];
    let mut largest = num_list[0];

    for num in num_list {
        if num > largest {
            largest = num;
        }
    }
    println!("The largest number is {}", largest);
}
*/
///Let's look at a program that finds the largest number
/// in two different lists.
fn largest(list: &[i32]) -> i32 {
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

    let num_list = vec![ 10,56,0,37,3,64];
    let result = largest(&num_list);
    println!("The largest number is {}", result);
}
/// In the above function, we remove duplication
/// by creating fn `largest` that loops through a given
/// i32 list and returns the largest number