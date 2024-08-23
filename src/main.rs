fn main() {
    // undertanding variable 
    variable();
    shadowing();
   
}

fn variable(){
    let mut x : i32 = 5 ;
    // let y: i32 ; warning: unused variable: `y` 
    // prefix it with an underscore: `_y`
    let _y: i32;
    
     assert_eq!(x,5);
     println!("X is Equal to 5");
 
    // x = x+9; cannot assign twice to immutable variable 
    // use keyword mut 
    x = x+9;
    assert_eq!(x,14);
    println!("X is Equal to 14");
}
 
/*
You can declare a new variable with the same name as a previous name .
*/
fn shadowing(){
    let x: i32 = 25;
    {
        let y : i32 = 52;
        let x : i32 =56;
        // value of x is 56 not 25
        println!("X value inside  this scope is {} and value of Y is {}",x,y);
    }
    assert_eq!(x,25);
    // re declare of x 
    let x = 12;
    println!("value of x is {}", x );
}
