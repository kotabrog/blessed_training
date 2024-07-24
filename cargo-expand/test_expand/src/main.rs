#[derive(Debug)]
struct S;

fn f() -> i32 {
    let x = 1;

    macro_rules! first_x {
        () => { x }
    }

    let x = 2;

    x + first_x!()
}

fn main() {
    println!("{:?}", S);
    println!("{:?}", f());
}
