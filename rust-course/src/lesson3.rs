pub fn ownership(){
    first_method();
}

struct Foo {
    x: i32
}
fn first_method() {
    //Value owner
    let foo = Foo {x:123};
    //Ownership moves to do_stuff
    do_stuff(foo);
    //This is not possable, ownership is moved to foo, value is deleted after it has run
    // do_stuff(foo);

    //This is possible, borrowed types aren't deleted
    let foo2 = Foo {x:123};
    do_stuff_borrows(&foo2);
    do_stuff_borrows(&foo2);
}

fn do_stuff(foo: Foo) {
    println!("do stuff: {}", foo.x)
}

fn do_stuff_borrows(foo : &Foo){
    println!("do stuff: {}",foo.x)
}