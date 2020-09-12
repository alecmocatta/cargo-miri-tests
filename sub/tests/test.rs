use std::env;
use std::path::PathBuf;

#[test]
fn current_dir() {
    println!("sub test");

    let env_dir = env::current_dir().unwrap();
    let crate_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    assert_eq!(env_dir.file_name().unwrap(), "sub");
    assert_eq!(env_dir, crate_dir);
}
