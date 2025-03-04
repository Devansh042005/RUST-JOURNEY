pub fn ownership(){
    let x:u8 = 3;
    let y:u8 = x;

    println!("{}",x);
    println!("{}",y);

    // In this case, x is copied to y and both x and y are valid
    // This is because u8 is stored in stack and it is cheap to copy the data
} 

pub fn ownership1(){
    let s1  = String::from("Hello");
    let s2  = s1.clone(); // clone is used to copy the data
    println!("{}",s1); // This will give an error because s1 is moved to s2
    println!("{}",s2);

    // Here ownership will be transferred to s2 and s1 will be invalid
    // This is because String is stored in heap and it is expensive to copy the data
}