// root of this library 
mod utilities;
mod functions;
mod function_tree;
mod antlr_parser;

mod visitor;
mod test_functions;
mod test_derivatives;







use function_tree::FnParser;
use functions::{AddFn, Function};

#[test] 
fn testing() {
    let f = AddFn::new("x", "y");

    let f_type = f.get_type();

    //println!("{:#?}", f_type);

    if f.depends_on("x") {
        println!("f depends on X !!!")
    }

    if f.depends_on("y") {
        println!("f depends on Y !!!")
    }


    println!("Hello, this is formating ")
}

