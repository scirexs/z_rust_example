use anyhow::{Result, Context, anyhow, bail, ensure};

fn main() -> Result<()> {
    err_handle_w_anyhow(&0)?;
    Ok(())
}
fn err_handle_w_anyhow(handle_type: &i32) -> Result<()> {
    let foo = "foo";
    let mut misc: Vec<String> = Vec::new();
    let conditional_expr = false;

    match *handle_type {
        1 => std::fs::read_to_string(foo).context("anyhow::Error is created with io::Error and this strings.")?,
        2 => std::fs::read_to_string(foo).with_context(|| format!("To avoid format cost etc., lazy eval is abled. {}", foo))?,
        3 => misc.pop().context("Option::None is casted to Result::Err.")?,
        _ => String::default(),
    };

    match *handle_type {
        4 => { return Err(anyhow!("Create anyhow::Error by manual like format: {}", foo)); },
        5 => { bail!("bail!({}) == return Err(anyhow!({}))", foo, foo); },
        6 => { ensure!(conditional_expr, "If conditional expr is false, bail!({})", foo); },
        _ => (),
    };
    Ok(())
}

// test with the command: cargo test -- --nocapture
#[cfg(test)]
mod tests{
    use crate::err_handle_w_anyhow;

    #[test]
    fn test_run() {
        for i in 1..7 {
            println!("---------- pattern {} ----------", i);
            if let Err(e) = err_handle_w_anyhow(&i) {
                println!("{:?}", e);
            }
        }
    }
}