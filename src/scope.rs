const GLOBAL_VAR: u8 = 100; // Global var will be present in the whole file;
pub fn scope_fn() {
    let x : u32 = 5;
    {
        let y: i32 = 10;
        println!("{}",x);
        println!("{}" , y);
        println!("{}",GLOBAL_VAR);
    }
    println!("{}",x);
    println!("{}",GLOBAL_VAR);

    // println!("{}",y); // This will give an error because y is not in scope
}