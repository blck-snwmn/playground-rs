use std::ops::{ControlFlow, Range};

fn flow<F>(inputs: Range<i32>, cond: F) -> ControlFlow<i32>
where
    F: Fn(i32) -> ControlFlow<i32>,
{
    for i in inputs {
        // ? が使える
        cond(i)?
    }
    ControlFlow::Continue(())
}
fn main() {
    {
        // 13 まで表示される
        let r = (2..100).try_for_each(|x| {
            println!("[try_for_each]{}", x);
            if 403 % x == 0 {
                return ControlFlow::Break(x);
            }

            ControlFlow::Continue(())
        });
        println!("{:?}", r)
    }
    {
        // 最後まで表示される。
        let r = (2..10).fold(ControlFlow::Continue(()), |acc, x| {
            println!("[fold]{}", x);
            if 7 % x == 0 {
                return ControlFlow::Break(x);
            }

            ControlFlow::Continue(())
        });
        println!("{:?}", r)
    }
    {
        let r = flow(1..10, |x| {
            println!("[flow1]{}", x);
            match x {
                xx if xx % 5 == 0 => ControlFlow::Break(xx),
                _ => ControlFlow::Continue(()),
            }
        });
        println!("{:?}", r) // Break(5)
    }
    {
        let r = flow(1..10, |x| {
            println!("[flow2]{}", x);
            match x {
                xx if xx % 100 == 0 => ControlFlow::Break(xx),
                _ => ControlFlow::Continue(()),
            }
        });
        println!("{:?}", r) // Continue(())
    }
}
