//! Block statement parsing tests.

use crate::syntax::{
    ast::{
        node::{
            Assign, Block, Call, Declaration, DeclarationList, FunctionDecl, Identifier, Node,
            Return, UnaryOp,
        },
        op, Const,
    },
    parser::tests::check_parser,
};

/// Helper function to check a block.
// TODO: #[track_caller]: https://github.com/rust-lang/rust/issues/47809
fn check_block<B>(js: &str, block: B)
where
    B: Into<Box<[Node]>>,
{
    check_parser(js, vec![Block::from(block.into()).into()]);
}

#[test]
fn empty() {
    check_block("{}", vec![]);
}

#[test]
fn non_empty() {
    check_block(
        r"{
            var a = 10;
            a++;
        }",
        vec![
            DeclarationList::Var(
                vec![Declaration::new_with_identifier(
                    "a",
                    Some(Const::from(10).into()),
                )]
                .into(),
            )
            .into(),
            UnaryOp::new(op::UnaryOp::IncrementPost, Identifier::from("a")).into(),
        ],
    );

    check_block(
        r"{
            function hello() {
                return 10
            }

            var a = hello();
            a++;
        }",
        vec![
            FunctionDecl::new(
                "hello".to_owned().into_boxed_str(),
                vec![],
                vec![Return::new(Const::from(10), None).into()],
            )
            .into(),
            DeclarationList::Var(
                vec![Declaration::new_with_identifier(
                    "a",
                    Node::from(Call::new(Identifier::from("hello"), vec![])),
                )]
                .into(),
            )
            .into(),
            UnaryOp::new(op::UnaryOp::IncrementPost, Identifier::from("a")).into(),
        ],
    );
}

#[test]
fn hoisting() {
    check_block(
        r"{
            var a = hello();
            a++;

            function hello() { return 10 }
        }",
        vec![
            FunctionDecl::new(
                "hello".to_owned().into_boxed_str(),
                vec![],
                vec![Return::new(Const::from(10), None).into()],
            )
            .into(),
            DeclarationList::Var(
                vec![Declaration::new_with_identifier(
                    "a",
                    Node::from(Call::new(Identifier::from("hello"), vec![])),
                )]
                .into(),
            )
            .into(),
            UnaryOp::new(op::UnaryOp::IncrementPost, Identifier::from("a")).into(),
        ],
    );

    check_block(
        r"{
            a = 10;
            a++;

            var a;
        }",
        vec![
            Assign::new(Identifier::from("a"), Const::from(10)).into(),
            UnaryOp::new(op::UnaryOp::IncrementPost, Identifier::from("a")).into(),
            DeclarationList::Var(vec![Declaration::new_with_identifier("a", None)].into()).into(),
        ],
    );
}
