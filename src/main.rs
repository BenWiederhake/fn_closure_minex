// fn_closure_minex – minimal example for something
// Copyright (C) 2017  Ben Wiederhake
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

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
