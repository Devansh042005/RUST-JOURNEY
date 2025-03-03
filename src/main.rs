fn main() {
    let num:u8 = 252;
    println!("This is stored in num {}", num);
    // rust uses unsigned integer

    let mut num1:u8 = 22;
    println!("This is num {}" ,num1);
    num1 = 222;
    println!("Updated num {}",num1);
    // mut is used to make the variable mutable

    //Strings
    
    // String --> Dynamic length strings - Heap Allocated
    //&str ---> Fized length strings - Special Memory

    let sentence: &str = "Hi, Devansh";
    // we can't add string in &str type
    println!("This is string literal {}",sentence);

    let  mut sentence1: String = String::from ("Hi, Devansh ser");
    sentence1.push_str(" What's up?????");
    println!("This is string literal {}",sentence1);

    // Tupple

    let emp_info:(&str,u8) = ("Ramesh" , 50);
    let emp_name = emp_info.0;
    let emp_age = emp_info.1;

    println!("Employee name = {} , Employee Age = {}" , emp_name , emp_age);

    let num2:u8 = 10;
    let num3:u8 = 12;
    let result:u8 = add(num2 , num3);
    println!("The sum of num2 and num3 is {}" , result);

}

fn add(item1:u8 , item2:u8) -> u8{
    return item1+item2;
}
