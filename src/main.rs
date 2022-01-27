
use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

fn main() {
    println!("请输入一个数字！");
    //生成一个随机数
    let secret_num: u32 = rand::thread_rng().gen_range(0,100);
    loop {
        let mut guess_num = String::new();
        // 用户输入一个数字 
        io::stdin().read_line(&mut guess_num).expect("有问题");
        let guess_num: u32 = match guess_num.trim().parse() {
            Ok(num)=>num,
            Err(_)=> {
                println!("请输入数字");
                continue;
            }
        };
        println!("你输入的随机数是：{}",guess_num);
        match guess_num.cmp(&secret_num){   
            Ordering::Less => println!("输入数字太小了"),
            Ordering::Equal =>{
                println!("you win!");
                break;
            },
            Ordering::Greater => println!("输入数字太大了"),
        }
    }
    
}
