extern crate clap;

use std::{thread, time};
use std::string::String;
use std::fmt::{self,Display,Formatter};

use clap::{Arg, App, SubCommand};

fn main() {
    println!("欢迎来到Rust动物园！");

    // 预定义已有动物
    let animals = vec![("猫",'🐱'),("狗",'🐕')];

    //定义产生生命的trait
    pub trait MakeLife {
        fn look(&self) -> char;
    }

    //定义动物struct
    struct Animal{
        name:String,
        looks:char,
        number:u32,
    }
    // 为animal实现MakeLife
    impl MakeLife for Animal {
        fn look(&self) -> char {
            self.looks
        }
    }
    impl Display for Animal{
        fn fmt(&self,f: &mut Formatter<'_>) -> fmt::Result{
            write!(f, "[{:#?}]", self.looks)
        }
    }

    let mut in_zoo_animal:Vec<Animal> = Vec::new();

    let matches = App::new("RustZoo")
        .version("0.1")
        .author("Wang Wenzhu <bcsxmail@163.com>")
        .about("一个Rust世界的动物园🐅🦁🐘🐒🐱🐀🐏")
        .arg(Arg::with_name("output")
            .help("设置一个输出文件")
            .required(false)
            .index(1))
        .subcommand(
            SubCommand::with_name("add")
                .about("添加动物")
                .arg(Arg::with_name("type")
                    .help("输入想添加的动物")
                    .required(false)
                    .index(1))
                .arg_from_usage("-n, --number '输入添加动物的数量'"),
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("查看动物园"),
        )
        .get_matches();

    if let Some(o) = matches.value_of("output") {
        println!("Value for output: {}", o);
    }

    
    if let Some(matches) = matches.subcommand_matches("add") {
        println!("现在开始迁入新的动物。");
        println!("{:?}",matches.value_of("type"));
        // match matches.value_of("type") {
        //     None => None,
        //     Some(i) => {
        //         println!("{:?}",String::from("i"));
        //         Some(i)
        //     },
        // }
        // add子命令执行
        if matches.is_present("type") {
            in_zoo_animal.push(
                Animal{
                    // name : String::from(matches.is_present("type")),
                    name : String::from("type"),
                    looks : animals[0].1,
                    number : 1,
                }
            );
            println!("{}来到了动物园{:#?}",in_zoo_animal[0],in_zoo_animal[0].look());
        } else {
            println!("\n请在参数中输入想要添加的动物。\n\n 比如：rust_zoo add 羊\n");
        }
    }
    if let Some(matches) = matches.subcommand_matches("show") {
        println!("现在开始展示");
        let mut number = 0;
        loop{
            while number <10 {
                let mut i = 1;
                while i < number {
                    print!(" ");
                    i = i+1;
                }
                //休息
                let ten_millis = time::Duration::from_millis(10);
                thread::sleep(ten_millis);
                print!("🐱");
                number = number + 1;
                print!("\r");
                
            }
            number = 0
        }
        
        
    }
    
    

}
