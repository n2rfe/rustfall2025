fn main() {
  
}

fn fahrenheit_to_celsius(t: i32){
    let f2c = t * (9/5) + 32;
    println!("The conversion of ", t, " F is ", f2c ," C {}");
}

fn celsius_to_fahrenheit(t: i32){
    let c2f = (t-32) * (5/9);
    println!("The conversion of ", t, " C is ", c2f, " F {}");
}

fn assignment(){
    let fp: i32 = 32;
    println!("The freezing point of water is: {}", fp);

    let mut t = 0;
    println!("The initial value of t is: {}", t);

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 5 {
            break counter;
        }
    f2c(t);
    };


    struct student{
        name: String,
        major: String,
    }

    impl student(
        fn new(s:u8,m)
    )
}
