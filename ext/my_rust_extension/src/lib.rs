use magnus::{function, define_module, Error};

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("ODBC")?;
    module.define_module_function("add", function!(add, 2))?;
    Ok(())
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
