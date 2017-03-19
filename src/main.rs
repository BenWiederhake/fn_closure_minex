// fn_closure_minex – minimal example for something
// Copyright (C) 2017  Ben Wiederhake
// Public Domain (Creative Commons CC0)

type LovelyFn = FnMut(u32);

pub struct Something<'a> {
    closure: &'a mut LovelyFn,
    value: u32,
}

pub fn do_work(something: &mut Something) {
    (something.closure)(something.value);
}

fn main() {
    let mut target: Vec<u32> = vec![];
    {
        let mut storer = |val: u32| { target.push(val) };
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
