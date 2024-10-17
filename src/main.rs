use my_parser_test::*;

pub fn main() {
    let parsed_data = list_parser::list("[1,1,2,3,5,8]");
    println!("{:?}", parsed_data);

    assert_eq!(list_parser::list("[1,1,2,3,5,8]"), Ok(vec![1, 1, 2, 3, 5, 8]));
}