fn main1(){
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter >= 10{
            break counter*2
        }
    }; 
    println!("{}",result);
}

fn main2(){
    'outer:loop{
        println!("outer loop");
        loop{
            println!("break outer loop");
            break 'outer;
        };
    };
}

fn main3(){
    let mut counter = 0;
    loop{
        counter += 1;
        if counter >5{
            continue;
        }
        println!("no continue");
    };
}


fn main4(){
    let mut counter = 0;
    while counter < 5 {
        if counter %2 == 0{
            println!("偶数");
        }else{
            println!("奇数");
        }
        counter +=1 ;
    }
}


fn main5(){
    for counter in 0 ..5{
        if counter %2 == 0{
            println!("偶数");
        }else{
            println!("奇数");
        }
    }

    for counter in 0 ..=4{
        if counter %2 == 0{
            println!("偶数");
        }else{
            println!("奇数");
        }
    }
}

fn main6(){
    let names = vec!["Curry","James","Durant"];
    for name in names.iter(){
        match name {
            &"Curry" => {
                println!("Best shooter")
            }
            _ =>{
                println!("Good player");
            }
        }
    }
}


fn main7(){
    let names = vec!["Curry","James","Durant"];

    for name in names.into_iter(){
        match name {
            "Curry" => {
                println!("Best shooter")
            }
            _ =>{
                println!("Good player");
            }
        }
    }
}


fn main(){
    main1();
    main2();
    main4();
    main5();
    main6();
    main7();
    main3();
    let mut names = vec!["Curry","James","Durant"];

    for name in names.iter_mut(){
        *name = match name {
            &mut "Curry" => {
                "Best shooter"
            }
            _ =>{
                "Good player"
            }
        }
    }
    println!("{:?}",names);
}
