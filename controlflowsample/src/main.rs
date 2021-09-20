use std::ops::ControlFlow;

fn main() {
    {
        // 13 まで表示される
        let r = (2..100).try_for_each(|x| {
            println!("[{}]", x);
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
            println!("[{}]", x);
            if 7 % x == 0 {
                return ControlFlow::Break(x);
            }

            ControlFlow::Continue(())
        });
        println!("{:?}", r)
    }
}
