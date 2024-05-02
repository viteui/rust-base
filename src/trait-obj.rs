pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Button {
   fn test(&self){
        println!("xx");
    }
}
pub trait Draw{
    fn draw(&self){
        println!(" 默认实现 ！！！！！！！");
    }
}

impl Draw for Button {
    // fn draw(&self) {
    //   // 绘制按钮的代码
    //     // println!("width: {}, height: {}", self.width, self.height)
    // }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 绘制SelectBox的代码
        println!("width: {}, height: {}", self.width, self.height)
    }
}


 impl SelectBox {
    fn test(&self){
        println!("xx");
    }
}


pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}


impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
            // 只能调用 特征方法
            // component.test();
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    let btn: Button = Button{
        width: 50,
        height: 10,
        label: String::from("OK11"),
    };
    btn.test();
}