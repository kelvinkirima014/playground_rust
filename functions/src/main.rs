fn main() {
    just_function(5);
}
// function parameters
// you need to declare the type of each parameter

fn just_function(x: i32) {
    println!("The value of x is {}", x);
}

// FUNCTION BODIES CONTAIN STATEMENTS AND EXPRESSIONS

/* Statements are instructions that perform some action and do not return a value
** Expressions evaluate to a resulting value
*/


//Creating a variable and assigning a value to it with`let` is a statement
fn create_statement(){
    // statement
    let k = 12;
    //This function is a statement itself
}
//as statement don't return values, you can't assign a `let` statement
// another variable: let x = (let y = 6); // will throw error
/* expressions evaluate to something and they make up most of Rust code
* 9+5 is an expression that evaluates to 11
*calling a function or a macro is an expression.
* The block we use to create new scopes is an expression, e.g
*/
fn exp_example (){
    let y = 5;
    let another_num ={
        let num =3;
        num + 1
    };
    /*this expression
    {
        let num = 3;
        num + 1
    }
    is a block that evaluates to 4. The value
    gets bound to `y` as part of the let statement
    ** note that `num + 1` is without a semicolon at the end,
    **expressions do not include ending semicolons, 
    if you add one, you turn it into a statement
*/
}
// FUNCTIONS WITH RETURN VALUES
/* functions can return values to the code that calls them
we don't name return values but we do declare their type after an arrow (->)
you can return early from a function by using the return keyword and
specifying a value, but most functions return the last expression implicitly
*/
fn five() -> i32 {

    5
// there are no function calls, macros or a `let` statement in this function
//this a perfectly valid Rust function
}
fn main_i () {
    let f = five();
    println!("The value of f is: {}", f);
}




