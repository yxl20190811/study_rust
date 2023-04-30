fn main() {
    //This is yy's first rust project
    println!("Hello, world!");
    /*so, muilt lines
    an so other
    */
    println!("{} is days", 30);
    //参数编号
    println!("{}, this is {0}, and this is {1}, {0} and {1} ", 50, 100);
    //参数名称
    println!("{s1},{s2},{s3}", s1="s3", s3="100", s2="s1");
    //参数的进制
    println!("base10:{0},base8:{0:o},base2:{0:b}, base16:{0:x}, BASE16:{0:X}", 1000);


    //左右填充
    println!("{number:>5}", number=1);
    println!("{number:0<5}", number=1);
    println!("{number:0>width$}", number=1, width=5);

    //下面的错误要被编译器捕捉到
    //println!("My name is {0}, {1} {0}", "Bond");


    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:9<width$}");
}
