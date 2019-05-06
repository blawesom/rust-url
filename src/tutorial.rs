fn main() {
    
    // Operation - + * / %
    let t = true;
    let f: bool = false; // with explicit type annotation
    
    let c = 'z';
    let ze = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    let spaces = "   ";
    let spaces = spaces.len();
    println!("There is {} spaces", spaces);

    println!("Summ is {}", add_function(5, 6));

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

    let nb = 2;
    let nb = nb + 1; // shadowing

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    println!("result is {}", binary_to_bool(0));


    let mut s = String::from("hello world");

    let word = first_word(&s); 

    let hello = &s[0..5];
    let world = &s[6..11];
    
    s.clear();
}


fn add_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    x + y
}


fn binary_to_bool(a: i8) -> bool {
    if a == 0 {
        false
    }
    else if a == 1 {
        true
    }
    else {
        false
    }
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

