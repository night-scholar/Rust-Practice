#[allow(dead_code)]
fn main1() {
    fn closure1(i:i32)->i32{i+1}
    let closure2 = |i:i32|i+1;
    let closure3 = |i:i32|->i32{i+1};
    let closure4 = ||2;

    let i = 1;
    println!("{}",closure1(i));
    println!("{}",closure2(i));
    println!("{}",closure3(i));
    println!("{}",closure4());
}
#[allow(dead_code)]
fn main2() {
    use std::mem;
    
    let color = "green";

    // 这个闭包打印 `color`。它会立即借用（通过引用，`&`）`color` 并将该借用和
    // 闭包本身存储到 `print` 变量中。`color` 会一直保持被借用状态直到
    // `print` 离开作用域。
    // `println!` 只需传引用就能使用，而这个闭包捕获的也是变量的引用，因此无需
    // 进一步处理就可以使用 `println!`。
    let print = || println!("`color`: {}", color);

    // 调用闭包，闭包又借用 `color`。
    print();
    print();

    let mut count = 0;

    // 这个闭包使 `count` 值增加。要做到这点，它需要得到 `&mut count` 或者
    // `count` 本身，但 `&mut count` 的要求没那么严格，所以我们采取这种方式。
    // 该闭包立即借用 `count`。
    //
    // `inc` 前面需要加上 `mut`，因为闭包里存储着一个 `&mut` 变量。调用闭包时，
    // 该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // 调用闭包。
    inc();
    inc();

    //let reborrow = &mut count;
    // ^ 试一试：将此行注释去掉。
    
    // 不可复制类型（non-copy type）。
    let movable = Box::new(3);

    // `mem::drop` 要求 `T` 类型本身，所以闭包将会捕获变量的值。这种情况下，
    // 可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动
    // （move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中。
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` 消耗了该变量，所以该闭包只能调用一次。
    consume();
    //consume();
    // ^ 试一试：将此行注释去掉。
}


#[allow(dead_code)]
fn main3() {
    // `Vec` 在语义上是不可复制的。
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));
 
    //println!("There're {} elements in vec", haystack.len());
    // ^ 取消上面一行的注释将导致编译时错误，因为借用检查不允许在变量被移动走
    // 之后继续使用它。
    
    // 在闭包的签名中删除 `move` 会导致闭包以不可变方式借用 `haystack`，因此之后
    // `haystack` 仍然可用，取消上面的注释也不会导致错误。
}




// Fn：表示捕获方式为通过引用（&T）的闭包
// FnMut：表示捕获方式为通过可变引用（&mut T）的闭包
// FnOnce：表示捕获方式为通过值（T）的闭包
//&T 只是获取了不可变的引用，&mut T 则可以改变 变量，T 则是拿到了变量的所有权而非借用。
#[allow(dead_code)]
fn main4() {
    use std::mem;
    // 该函数将闭包作为参数并调用它。
    fn apply<F>(f: F) where
        // 闭包没有输入值和返回值。
        // FnOnce：表示捕获方式为通过值（T）的闭包
        F: FnOnce() {
        // ^ 试一试：将 `FnOnce` 换成 `Fn` 或 `FnMut`。

        f();
    }
    // 输入闭包，返回一个 `i32` 整型的函数。
    fn apply_to_3<F>(f: F) -> i32 where
        // 闭包处理一个 `i32` 整型并返回一个 `i32` 整型。
        F: Fn(i32) -> i32 {

        f(3)
    }
    let greeting = "hello";
    // 不可复制的类型。
    // `to_owned` 从借用的数据创建有所有权的数据==String::from("goodbye")。
    let mut farewell = "goodbye".to_owned();

    // 捕获 2 个变量：通过引用捕获 `greeting`，通过值捕获 `farewell`。
    let diary = || {
        // `greeting` 通过引用捕获，故需要闭包是 `Fn`。
        println!("I said {}.", greeting);

        // 下文改变了 `farewell` ，因而要求闭包通过可变引用来捕获它。
        // 现在需要 `FnMut`。
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // 手动调用 drop 又要求闭包通过值获取 `farewell`。
        // 现在需要 `FnOnce`。
        mem::drop(farewell);
    };

    // 以闭包作为参数，调用函数 `apply`。
    apply(diary);

    // 闭包 `double` 满足 `apply_to_3` 的 trait 约束。
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

#[allow(dead_code)]
fn main5() {
    // 输入闭包，返回一个 `i32` 整型的函数。
    fn apply_to_3<F>(f: F) -> i32 where
        // 闭包处理一个 `i32` 整型并返回一个 `i32` 整型。
        F: Fn(i32) -> i32 {

        f(3)
    }

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

#[derive(Debug)]
struct E {
    a: String,
}

impl Drop for E {
    fn drop(&mut self) {
        println!("destroyed struct E");
    }
}
#[allow(dead_code)]
fn main6() {
    fn fn_once<F>(func: F) where F: FnOnce() {
        println!("fn_once begins");
        func();
        println!("fn_once ended");
    }
    
    let e = E { a: "fn_once".to_string() };
    // 这样加个move，看看程序执行输出顺序有什么不同
    let f = move || println!("fn once calls: {:?}", e);
    // let f = || println!("fn once closure calls: {:?}", e);
    fn_once(f);
    println!("main ended");
}

#[allow(dead_code)]
fn main7() {
    fn fn_mut<F>(mut func: F) where F: FnMut() {
        println!("fn_mut begins");
        func();
        func();
        println!("fn_mut ended");
    }
    let mut e = E { a: "fn_once".to_string() };
	let f = || { 
        println!("FnMut closure calls: {:?}", e); 
        e.a = "fn_mut".to_string(); 
    };
    fn_mut(f);
    println!("main ended");
}

#[allow(dead_code)]
fn main8() {
    fn fn_fn<F>(func: F) where F: Fn() {
        println!("fn begins");
        func();
        func();
        println!("fn ended");
    }
    let e = E { a: "fn".to_string() };
	let f = || { println!("Fn closure calls: {:?}", e); };
    fn_fn(f);
    println!("main ended");
}


//Iterator::any
#[allow(array_into_iter)]
#[allow(dead_code)]
fn main9() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec 的 `iter()` 举出 `&i32`。（通过用 `&x` 匹配）把它解构成 `i32`。
    // 译注：注意 `any` 方法会自动地把 `vec.iter()` 举出的迭代器的元素一个个地
    // 传给闭包。因此闭包接收到的参数是 `&i32` 类型的。
    // pub trait Iterator {
    //     // 被迭代的类型。
    //     type Item;
    
    //     // `any` 接受 `&mut self` 参数（译注：回想一下，这是 `self: &mut Self` 的简写）
    //     // 表明函数的调用者可以被借用和修改，但不会被消耗。
    //     fn any<F>(&mut self, f: F) -> bool where
    //         // `FnMut` 表示被捕获的变量最多只能被修改，而不能被消耗。
    //         // `Self::Item` 指明了被捕获变量的类型（译注：是迭代器的元素本身的类型）
    //         F: FnMut(Self::Item) -> bool {}
            
    //         // 译注：原文说 `Self::Item` 表明变量是通过值传递给闭包的，这是说错了。
    //         // `FnMut` 就表示闭包只能通过引用捕获变量。把类型为 `T` 的变量作为闭包
    //         // 的参数不代表闭包会拿走它的值，也可能是拿走它的引用。
    // }
    println!("2 in printlnvec1: {}", vec1.iter().any(|&x| x == 2));
    // 对 vec 的 `into_iter()` 举出 `i32` 类型。无需解构。
    println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组的 `iter()` 举出 `&i32`。
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // 对数组的 `into_iter()` 通常举出 `&i32`。
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
}


//Iterator::find
#[allow(dead_code)]
#[allow(array_into_iter)]
fn main10() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec1 的 `iter()` 举出 `&i32` 类型。
    let mut iter = vec1.iter();
    // 对 vec2 的 `into_iter()` 举出 `i32` 类型。
    let mut into_iter = vec2.into_iter();
    // 对迭代器举出的元素的引用是 `&&i32` 类型。解构成 `i32` 类型。
    // 译注：注意 `find` 方法会把迭代器元素的引用传给闭包。迭代器元素自身
    // 是 `&i32` 类型，所以传给闭包的是 `&&i32` 类型。
    // pub trait Iterator {
    //     // 被迭代的类型。
    //     type Item;
    
    //     // `find` 接受 `&mut self` 参数，表明函数的调用者可以被借用和修改，
    //     // 但不会被消耗。
    //     fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
    //         // `FnMut` 表示被捕获的变量最多只能被修改，而不能被消耗。
    //         // `&Self::Item` 指明了被捕获变量的类型（译注：是对迭代器元素的引用类型） 
    //         P: FnMut(&Self::Item) -> bool {}
    // }
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    // 对迭代器举出的元素的引用是 `&i32` 类型。解构成 `i32` 类型。
    println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组的 `iter()` 举出 `&i32`。
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    // 对数组的 `into_iter()` 通常举出 `&i32``。
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&&x| x == 2));
}


//高阶函数的函数式写法。
#[allow(dead_code)]
fn main11() {
    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // 命令式（imperative）的写法
    // 声明累加器变量
    let mut acc = 0;
    // 迭代：0，1, 2, ... 到无穷大
    for n in 0.. {
        // 数字的平方
        let n_squared = n * n;

        if n_squared >= upper {
            // 若大于上限则退出循环
            break;
        } else if is_odd(n_squared) {
            // 如果是奇数就计数
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // 函数式的写法
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)             // 所有自然数取平方
             .take_while(|&n| n < upper) // 取小于上限的
             .filter(|&n| is_odd(n))     // 过滤器，取奇数
             .fold(0, |sum, i| sum + i); // 最后加起来
    println!("functional style: {}", sum_of_squared_odd_numbers);
}

//发散函数
#[allow(dead_code)]
fn main() {
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        //0..up_to不包含up_to。0..=up_to包含up_to。
        for i in 0..up_to {
            // 注意这个 match 表达式的返回值必须为 u32，
            // 因为 “addition” 变量是这个类型。
            let addition: u32 = match i%2 == 1 {
                // “i” 变量的类型为 u32，这毫无问题。
                true => i,
                // 另一方面，“continue” 表达式不返回 u32，但它仍然没有问题，
                // 因为它永远不会返回，因此不会违反匹配表达式的类型要求。
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}