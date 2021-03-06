//ignore-windows: TODO env var emulation stubbed out on Windows

use std::env;

fn main() {
    assert_eq!(env::var("MIRI_TEST"), Err(env::VarError::NotPresent));
    env::set_var("MIRI_TEST", "the answer");
    assert_eq!(env::var("MIRI_TEST"), Ok("the answer".to_owned()));
}
