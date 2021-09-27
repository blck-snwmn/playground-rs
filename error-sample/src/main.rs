use anyhow::{Context, Result};
fn main() -> Result<()> {
    do_1().context("aaaaaa")
}

fn do_1() -> Result<()> {
    do_2().with_context(|| "with_context in do_1".to_string())
}

fn do_2() -> Result<()> {
    Err(anyhow::anyhow!("failed to do_2").context("context in do_2"))
}
