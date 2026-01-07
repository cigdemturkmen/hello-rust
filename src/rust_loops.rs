pub fn sia_chandelier (){
    let mut while_count = 0;
    while while_count < 3 {
            let mut loop_count = 1;
        loop {
            print!("{}, ", loop_count);
            loop_count += 1;
            if loop_count > 3 {
                 println!("Drink!");
                break;
            }
        }
        while_count += 1;
    }
    print!("Throw 'em back 'til I lose count.");
}

pub fn while_loop() {
    let mut count = 1;

    while count <= 5 {
        println!("Count: {}", count);
        count += 1;
    }
}

pub fn for_loop() {
    let numbers = [10, 20, 30, 40, 50];

    for num in numbers.iter() {
        println!("Number: {}", num);
    }

    // or you can simply give an range like below
    for i in 1..6 {
        println!("i is: {}", i);
    }
}

pub fn loops() {
    let mut count = 1;

    // It will run FOREVER unless break is called inside the loop
    loop {
        println!("Hello World!");

        if count == 3 {
            break;
        }

        count += 1;
    }
}
