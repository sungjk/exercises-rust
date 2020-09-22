#[derive(Debug)] 
enum FunList<T> {
    Nil,
    Cons(T, Box<FunList<T>>)
}

impl<T> FunList<T> {
    fn is_empty(&self) -> bool {
        match self {
            FunList::Nil => true,
            FunList::Cons(_, _) => false,
        }
    }
    
    fn map<R>(&self, func: fn(&T) -> R) -> FunList<R> {
        match self {
            FunList::Nil => FunList::Nil,
            FunList::Cons(head, tail) => FunList::Cons(
                func(&head),
                Box::new(tail.map(func)),
            ),
        }
    }

    fn reduce<R>(&self, func: fn(R, &T) -> R, init: R) -> R {
        match self {
            FunList::Nil => init,
            FunList::Cons(head, tail) => func(tail.reduce(func, init), &*head),
        }
    }
}

fn main() {
    let nil: FunList<i32> = FunList::Nil;
    let funlist = FunList::Cons(
        1, Box::new(FunList::Cons(
            2, 
            Box::new(FunList::Cons(
                3, 
                Box::new(FunList::Nil))))));
    println!("Nil: {:?}", nil);
    println!("funlist: {:?}", funlist);
    println!("Nil.is_empty: {}", nil.is_empty());
    println!("funlist.is_empty: {}", funlist.is_empty());

    let mapped = funlist.map(|i| i * 10);
    println!("funlist.map(*10): {:?}", mapped);

    let reduced = funlist.reduce(|acc, iter| acc + iter, 0);
    println!("funlist.reduce(+, 0): {:?}", reduced);
}
