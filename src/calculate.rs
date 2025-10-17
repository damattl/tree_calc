use crate::{
    token::{Operator, Token},
    traits::Numeric,
    tree::BinaryTree,
};

pub fn calculate_result<T: Numeric>(parsed_term: &BinaryTree<Token<T>>) -> T {
    let mut result: Vec<T> = vec![];
    parsed_term.traverse_postorder(0, &mut |node| {
        let (_, el) = node;
        if el.is_none() {
            return;
        }
        // println!("{el:?}");
        // println!("{result:#?}");

        match el.unwrap() {
            Token::Op(Operator::ADD) => {
                let a = result[result.len() - 2];
                let b = result[result.len() - 1];
                result.pop();
                result.pop();
                result.push(a + b);
            }
            Token::Op(Operator::MUL) => {
                let a = result[result.len() - 2];
                let b = result[result.len() - 1];
                result.pop();
                result.pop();
                result.push(a * b);
            }
            Token::Value(val) => result.push(*val),
            Token::Empty => {
                println!("None");
            }
            other => println!("Shouldn't be here but is here: {:#?}", other),
        };
    });

    println!("Result: {result:#?}");
    *result.get(0).unwrap_or(&T::default())
}
