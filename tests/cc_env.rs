extern crate tempdir;
extern crate cc;

use std::env;

mod support;
use support::Test;

#[test]
fn main() {
    ccache();
    distcc();
    ccache_spaces();
    ccache_env_flags();
}

fn ccache() {
    let test = Test::gnu();
    test.shim("ccache");

    env::set_var("CC", "ccache lol-this-is-not-a-compiler foo");
    test.gcc().file("foo.c").compile("libfoo.a");

    test.cmd(0)
        .must_have("lol-this-is-not-a-compiler foo")
        .must_have("foo.c")
        .must_not_have("ccache");
}

fn ccache_spaces() {
    let test = Test::gnu();
    test.shim("ccache");

    env::set_var("CC", "ccache        lol-this-is-not-a-compiler foo");
    test.gcc().file("foo.c").compile("libfoo.a");
    test.cmd(0).must_have("lol-this-is-not-a-compiler foo");
}

fn distcc() {
    let test = Test::gnu();
    test.shim("distcc");

    env::set_var("CC", "distcc lol-this-is-not-a-compiler foo");
    test.gcc().file("foo.c").compile("libfoo.a");

    test.cmd(0)
        .must_have("lol-this-is-not-a-compiler foo")
        .must_have("foo.c")
        .must_not_have("distcc");
}

fn ccache_env_flags() {
    use std::path::Path;
    use std::ffi::OsString;

    let test = Test::gnu();
    test.shim("ccache");

    env::set_var("CC", "ccache lol-this-is-not-a-compiler");
    let compiler = test.gcc().file("foo.c").get_compiler();
    assert_eq!(compiler.path(), Path::new("ccache"));
    assert_eq!(compiler.cc_env(), OsString::from("ccache lol-this-is-not-a-compiler"));
    assert!(compiler.cflags_env().into_string().unwrap().contains("ccache") == false);
    assert!(compiler.cflags_env().into_string().unwrap().contains(" lol-this-is-not-a-compiler") == false);

    env::set_var("CC", "");
}
