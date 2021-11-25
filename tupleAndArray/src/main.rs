fn main() {
    //元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let six_point_four = tup.1;
    println!("The value of y is: {}", y);
    println!("The value of six_point_four is: {}", six_point_four);

    //数组
    let a = [
        "January",
        "February",
        "March",
        "April",
        "May", 
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("a[0] : {}", a[0]);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a[0] : {}", a[0]);

    let a = [3; 5];
    println!("a : {},{},{},{},{}", a[0], a[1], a[2], a[3], a[4]);
}
