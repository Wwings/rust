// enum 关键字允许创建一个代表数个可能变量的数据的类型
// 在 struct 中任何合法的变量在 enum 中同样合法


//隐藏未使用代码警告的属性
#![allow(dead_code)]

// 创建一个 'enum' 枚举来划分人的类别，注意命名和类型的信息是如何明确规定变量的
// 


enum Person {
    // 一个 'enum' 可能是个 'unit-like' （类单元结构体）
    Engineer,
    Scientist,
    // 或 像是个元组结构体
    Height(i32),
    Weight(i32),
    // 或 像是个普通的结构体
    Info { name: String, height: i32 },
}

// 此函数将Person 'enum' 作为一个参数，无返回值
fn inspect(p: Person) {
    // 'enum' 的使用必须覆盖所有情形（这是不可置疑的），所以使用 match 以分支方式覆盖所有类型
    match p {
        Person::Engineer => println!("Is a Engineer"),
        Person::Scientist => println!("Is a Scientist"),
        // 从 'enum' 内部解构 'i'
        Person::Height(i) => println!("has a height of {}", i),
        Person::Weight(i) => println!("has a weight of {}", i),
        // 将 'Info' 解构成 'name' 和 'height'
        Person::Info{ name, height } => {
            println!("{} is {} tall", name, height);
        },
    } 
}

fn enum_demo() {

    let person = Person::Height(18);
    let amira = Person::Weight(10);
    // 'to_owned()' 从一个字符串 slice 创建一个具有所有权的 'String'
    let dave = Person::Info{ name: "Dave".to_owned(), height: 72 };
    let rebecca = Person::Scientist;
    let roben = Person::Engineer;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(roben);

}




// enum 的最常见用法就是创建链表（Linked-List）

use List::*;

enum List {
    // Cons: 元组结构体，包含一个节点元素以及指向下一个元素的指针
    Cons(u32, Box<List>),
    // Nil: 末指针，表明链表结束
    Nil,
}

// 方法可以在 enum 中定义
impl List {
    // 创建一个空链表
    fn new() -> List {
        //Nil 为 List 类型
        Nil
    }

    // 处理一个列表，得到一个头部带上一个新元素的同样类型的列表并返回此值
    fn prepend(self, elem: u32) -> List {
        // 'Cons' 同样为 List 类型
        Cons(elem, Box::new(self))
    }

    //返回列表的长度
    fn len(&self) -> u32 {
        // 'self' 必须匹配，因为这个方法的行为取决于 'self' 的变化类型
        // 'self' 为 '&List' 类型，所以 '*self' 为 'List' 类型，一个具体的 'T' 类型的匹配要参考引用 '&T' 的匹配
        match *self {
            // 不能得到 tail 的所有权，因为 self 是借用的
            // 而是得到 tail 的一个引用
            Cons(_, ref tail) => 1 + tail.len(),
            // 基本情况，空列表的长度为0
            Nil => 0,
        }
    }
}

fn link_enum_demo() {
    // 创建一个空列表
    let mut list = List::new();

    // 追加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    //显示链表的最终状态
    println!("linklist has {} elements", list.len());
    //println!("{}", list.stringify());   // method `stringify` not found for this

}





fn main() {

    link_enum_demo();

    //enum_demo();

    println!("Hello, world!");
}
