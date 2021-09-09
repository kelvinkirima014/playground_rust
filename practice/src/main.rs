//ARRAYS
/*fn main() {
    let arr1 = [1,2,3];
    let arr2: [u8; 3] = [1, 2, 3,];
    let arr_of_fours = [4; 3];
    let blanks: [u8; 3] = [0; 3];

    let arrays = [arr1,arr_of_fours, arr2, blanks];

    for i in &arrays {
        println!("{:?}", i);

        for j in i {
            println!("\t{:?} + 10 = {:?}", j, j+10);
        }

        let mut sum = 0;
        for a in 0..i.len() {
            sum += i[a];
        }
        println!("\t$ {:?} = {:?}", i, sum);
    }
}
*/
fn main() {
    let array = [2,7,9,20,34,44];
    let num: usize = 9;
    for i in 0..array.len() {
        if i == num {
            
        }
        println!("found num at {}", i);
    }
    
}