/*
    Rustaceans say that the first variable is shadowed byt he second, which means
        that the second variable's value is what appears when the variable is used.
    We can shadow a variable by using rhe same variable's name and reapeating the
        use of the let keyword as follows.
    Shadowing is different from marking a variable as mut, because we'll get a
        compile-time error if we accidentally try to reassign to this variable
        without using the let keyword. By using let, we can perform a few
        transformations on a value but have the variable be immutable after
        those transformations have been completed.
    The other difference between mut and shadowing is that because we’re
        effectively creating a new variable when we use the let keyword again,
        we can change the type of the value but reuse the same name.
*/

fn main() {
    let x = 5;
    
    let x = x + 1;
    
    let x = x * 2;
    
    println!("the value of x is: {}", x);
}
