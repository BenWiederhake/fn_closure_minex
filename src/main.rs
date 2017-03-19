// fn_closure_minex – minimal example for something
// Copyright (C) 2017  Ben Wiederhake
// Public Domain (Creative Commons CC0)

/*
 * Modify the source to enable VERSION A or VERSION B and
 * try to build each with `cargo build`.
 *
 * With VERSION A, everything is fine.
 * With VERSION B, the compiler throws error E0373
 * ("closure may outlive the current function, but it borrows `target`,
 *   which is owned by the current function")
 *
 * Did I misunderstand something about type aliasing, or is there a bug
 * with the compiler?
 */

/* VERSION A */
pub struct Something<'a> {
    closure: &'a mut FnMut(u32),
    value: u32,
}
// */

/* VERSION B
type LovelyFn = FnMut(u32);
pub struct Something<'a> {
    closure: &'a mut LovelyFn,
    value: u32,
}
// */

pub fn do_work(something: &mut Something) {
    (something.closure)(something.value);
}

fn main() {
    let mut target: Vec<u32> = vec![];
    {
        let mut storer = |val: u32| { target.push(val) };
        //               ^^^^^^^^^^   ------ `target` is borrowed here
        //               |
        //               may outlive borrowed value `target`
        let mut ctx = Something{
            closure: &mut storer,
            value: 123,
        };
        do_work(&mut ctx);
        ctx.value = 234;
        do_work(&mut ctx);
    }

    assert_eq!(vec![123, 234], target);
}
