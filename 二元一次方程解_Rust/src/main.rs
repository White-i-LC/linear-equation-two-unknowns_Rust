use std::io;

fn main() {
    loop {
        int();
        println!("输入E可退出,输入其他任意字符可继续计算");
        let mut key = String::new();
        io::stdin().read_line(&mut key)
            .expect("Failed to read line");
        let key = key.trim();
        if key == "E" {
            break
        }
    }
}

fn int(){

    //First equation
    println!("请输入第一个方程（方程示例：ax + by = c）注意：解是x和y");
    println!("请输入a的值");
    let mut onea = String::new();
    io::stdin().read_line(&mut onea)
        .expect("Failed to read line");
    let a1: f64 = onea.trim().parse()
        .expect("0");

    println!("请输入b的值");
    let mut oneb = String::new();
    io::stdin().read_line(&mut oneb)
        .expect("Failed to read line");
    let b1: f64 = oneb.trim().parse()
        .expect("0");

    println!("请输入c的值");
    let mut onec = String::new();
    io::stdin().read_line(&mut onec)
        .expect("Failed to read line");
    let c1: f64 = onec.trim().parse()
        .expect("0");


    //Second equation
    println!("请输入第二个方程（方程示例：ax + by = c）注意：解是x和y");
    println!("请输入a的值");
    let mut twoa = String::new();
    io::stdin().read_line(&mut twoa)
        .expect("Failed to read line");
    let a2: f64 = twoa.trim().parse()
        .expect("0");

    println!("请输入b的值");
    let mut twob = String::new();
    io::stdin().read_line(&mut twob)
        .expect("Failed to read line");
    let b2: f64 = twob.trim().parse()
        .expect("0");

    println!("请输入c的值");
    let mut twoc = String::new();
    io::stdin().read_line(&mut twoc)
        .expect("Failed to read line");
    let c2: f64 = twoc.trim().parse()
        .expect("0");
    calculation(a1,b1,c1,a2,b2,c2);
}

fn calculation(a1:f64,b1:f64,c1:f64,a2:f64,b2:f64,c2:f64){

    let x1 = a1 * c2 - a2 * c1;
    let x2 = b2 * a1 - a2 * b1;
    let x = x1 / x2;
    println!("x的值是:{}", x);

    let y1 = a1 * c2 - a2 * c1;
    let y2 = b2 * a1 - a2 * b1;
    let y = y1 / y2;
    println!("y的值是:{}", y);
}

