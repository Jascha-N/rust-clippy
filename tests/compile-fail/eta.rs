#![feature(plugin)]
#![plugin(clippy)]
#![allow(unknown_lints, unused, no_effect, redundant_closure_call, many_single_char_names)]
#![deny(redundant_closure)]

fn main() {
    let a = Some(1u8).map(|a| foo(a));
    //~^ ERROR redundant closure found
    //~| HELP remove closure as shown
    //~| SUGGESTION let a = Some(1u8).map(foo);
    meta(|a| foo(a));
    //~^ ERROR redundant closure found
    //~| HELP remove closure as shown
    //~| SUGGESTION meta(foo);
    let c = Some(1u8).map(|a| {1+2; foo}(a));
    //~^ ERROR redundant closure found
    //~| HELP remove closure as shown
    //~| SUGGESTION let c = Some(1u8).map({1+2; foo});
    let d = Some(1u8).map(|a| foo((|b| foo2(b))(a))); //is adjusted?
    all(&[1, 2, 3], &&2, |x, y| below(x, y)); //is adjusted
    unsafe {
        Some(1u8).map(|a| unsafe_fn(a)); // unsafe fn
    }

    // See #515
    let a: Option<Box<::std::ops::Deref<Target = [i32]>>> =
        Some(vec![1i32, 2]).map(|v| -> Box<::std::ops::Deref<Target = [i32]>> { Box::new(v) });
}

fn meta<F>(f: F) where F: Fn(u8) {
    f(1u8)
}

fn foo(_: u8) {

}

fn foo2(_: u8) -> u8 {
    1u8
}

fn all<X, F>(x: &[X], y: &X, f: F) -> bool
where F: Fn(&X, &X) -> bool {
    x.iter().all(|e| f(e, y))
}

fn below(x: &u8, y: &u8) -> bool { x < y }

unsafe fn unsafe_fn(_: u8) { }
