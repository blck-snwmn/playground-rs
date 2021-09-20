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
        assert_eq!(r, ControlFlow::Break(13));
    }
}
