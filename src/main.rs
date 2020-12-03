fn main() {
    list::rc();
    list_mut::ref_cell();
}

mod list {
    use std::rc::Rc;
    use List::{Cons, Nil};

    // Rc という参照カウントが 0 になるまで解放されない値を使えば、誰かが明示的に所有権を持つことなく
    // 不要になれば解放される値を作れる
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    pub fn rc() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));

        let _b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));

        {
            let _c = Cons(3, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }

        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
}

mod list_mut {
    use std::cell::RefCell;
    use std::rc::Rc;
    use List::{Cons, Nil};

    // 可変参照を返す RefCell を複数箇所から参照を共有できる Rc で共有することで
    // 可変な値を複数箇所から同時に変更することができる。
    // ただし実行時に借用チェックが入り、それに引っかかるとパニックが発生する。
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    pub fn ref_cell() {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        // a, b, c それぞれが参照している value の値がすべて 15 に変更されている
        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
}
