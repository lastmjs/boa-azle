use crate::syntax::{
    ast::{
        node::{
            declaration::{BindingPatternTypeArray, BindingPatternTypeObject},
            Block, Catch, Declaration, DeclarationList, Finally, Try,
        },
        Const,
    },
    parser::tests::{check_invalid, check_parser},
};

#[test]
fn check_inline_with_empty_try_catch() {
    check_parser(
        "try { } catch(e) {}",
        vec![Try::new(
            vec![],
            Some(Catch::new(
                Declaration::new_with_identifier("e", None),
                vec![],
            )),
            None,
        )
        .into()],
    );
}

#[test]
fn check_inline_with_var_decl_inside_try() {
    check_parser(
        "try { var x = 1; } catch(e) {}",
        vec![Try::new(
            vec![DeclarationList::Var(
                vec![Declaration::new_with_identifier(
                    "x",
                    Some(Const::from(1).into()),
                )]
                .into(),
            )
            .into()],
            Some(Catch::new(
                Declaration::new_with_identifier("e", None),
                vec![],
            )),
            None,
        )
        .into()],
    );
}

#[test]
fn check_inline_with_var_decl_inside_catch() {
    check_parser(
        "try { var x = 1; } catch(e) { var x = 1; }",
        vec![Try::new(
            vec![DeclarationList::Var(
                vec![Declaration::new_with_identifier(
                    "x",
                    Some(Const::from(1).into()),
                )]
                .into(),
            )
            .into()],
            Some(Catch::new(
                Declaration::new_with_identifier("e", None),
                vec![DeclarationList::Var(
                    vec![Declaration::new_with_identifier(
                        "x",
                        Some(Const::from(1).into()),
                    )]
                    .into(),
                )
                .into()],
            )),
            None,
        )
        .into()],
    );
}

#[test]
fn check_inline_with_empty_try_catch_finally() {
    check_parser(
        "try {} catch(e) {} finally {}",
        vec![Try::new(
            vec![],
            Some(Catch::new(
                Declaration::new_with_identifier("e", None),
                vec![],
            )),
            Some(Finally::from(vec![])),
        )
        .into()],
    );
}

#[test]
fn check_inline_with_empty_try_finally() {
    check_parser(
        "try {} finally {}",
        vec![Try::new(vec![], None, Some(Finally::from(vec![]))).into()],
    );
}

#[test]
fn check_inline_with_empty_try_var_decl_in_finally() {
    check_parser(
        "try {} finally { var x = 1; }",
        vec![Try::new(
            vec![],
            None,
            Some(Finally::from(vec![DeclarationList::Var(
                vec![Declaration::new_with_identifier(
                    "x",
                    Some(Const::from(1).into()),
                )]
                .into(),
            )
            .into()])),
        )
        .into()],
    );
}

#[test]
fn check_inline_empty_try_paramless_catch() {
    check_parser(
        "try {} catch { var x = 1; }",
        vec![Try::new(
            Block::from(vec![]),
            Some(Catch::new::<_, Declaration, _>(
                None,
                vec![DeclarationList::Var(
                    vec![Declaration::new_with_identifier(
                        "x",
                        Some(Const::from(1).into()),
                    )]
                    .into(),
                )
                .into()],
            )),
            None,
        )
        .into()],
    );
}

#[test]
fn check_inline_with_binding_pattern_object() {
    check_parser(
        "try {} catch ({ a, b: c }) {}",
        vec![Try::new(
            Block::from(vec![]),
            Some(Catch::new::<_, Declaration, _>(
                Some(Declaration::new_with_object_pattern(
                    vec![
                        BindingPatternTypeObject::SingleName {
                            ident: "a".into(),
                            property_name: "a".into(),
                            default_init: None,
                        },
                        BindingPatternTypeObject::SingleName {
                            ident: "c".into(),
                            property_name: "b".into(),
                            default_init: None,
                        },
                    ],
                    None,
                )),
                vec![],
            )),
            None,
        )
        .into()],
    );
}

#[test]
fn check_inline_with_binding_pattern_array() {
    check_parser(
        "try {} catch ([a, b]) {}",
        vec![Try::new(
            Block::from(vec![]),
            Some(Catch::new::<_, Declaration, _>(
                Some(Declaration::new_with_array_pattern(
                    vec![
                        BindingPatternTypeArray::SingleName {
                            ident: "a".into(),
                            default_init: None,
                        },
                        BindingPatternTypeArray::SingleName {
                            ident: "b".into(),
                            default_init: None,
                        },
                    ],
                    None,
                )),
                vec![],
            )),
            None,
        )
        .into()],
    );
}

#[test]
fn check_inline_invalid_catch() {
    check_invalid("try {} catch");
}

#[test]
fn check_inline_invalid_catch_without_closing_paren() {
    check_invalid("try {} catch(e {}");
}

#[test]
fn check_inline_invalid_catch_parameter() {
    check_invalid("try {} catch(1) {}");
}

#[test]
fn check_invalid_try_no_catch_finally() {
    check_invalid("try {} let a = 10;");
}

#[test]
fn check_invalid_catch_with_empty_paren() {
    check_invalid("try {} catch() {}");
}

#[test]
fn check_invalid_catch_with_duplicate_params() {
    check_invalid("try {} catch({ a, b: a }) {}");
}

#[test]
fn check_invalid_catch_with_lexical_redeclaration() {
    check_invalid("try {} catch(e) { let e = 'oh' }");
}

#[test]
fn check_invalid_catch_with_var_redeclaration() {
    check_invalid("try {} catch(e) { var e = 'oh' }");
}
