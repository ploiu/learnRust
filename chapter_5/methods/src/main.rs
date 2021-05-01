#[derive(Debug)]
struct Rect {
    width: i32,
    height: i32,
}

// we can define methods for a struct in an impl block
impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    // associated functions are declared in impl as well, but don't take `self`
    fn from(width: i32, height: i32) -> Rect {
        Rect { width, height }
    }

    // sadly, rust does not support overloads
    fn from_rect(rect: &Rect) -> Rect {
        Rect {
            width: rect.width,
            height: rect.height,
        }
    }
}

fn main() {
    // we can create a rect and call its functions like we would in any other language
    let rect = Rect {
        width: 30,
        height: 40,
    };
    println!("rect: {:?}", rect);
    println!("area: {}", rect.area());
    let new_rect = Rect::from(10, 10);
    println!("new rect: {:?}", new_rect);
}
