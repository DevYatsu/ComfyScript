use hashbrown::HashMap;

use crate::parser::ast::ASTNode;

#[derive(Debug, Clone, PartialEq)]
struct SymbolTable {
    pub functions: HashMap<String, ASTNode>,
    pub variables: HashMap<String, ASTNode>,
}

pub fn interpret(program: ASTNode) {
    let nodes = match program {
        ASTNode::Program { body } => body,
        _ => unreachable!(),
    };

    for node in nodes {
        match node {
            ASTNode::Program { body } => todo!(),
            ASTNode::ImportDeclaration { specifiers, source } => todo!(),
            ASTNode::VariableDeclaration { declarations, kind } => todo!(),
            ASTNode::ExpressionStatement { expression } => todo!(),
            ASTNode::FunctionDeclaration {
                id,
                params,
                body,
                return_type,
                is_shortcut,
            } => todo!(),
            ASTNode::ForStatement {
                declarations,
                kind,
                source,
                body,
            } => todo!(),
            ASTNode::WhileStatement { test, body } => todo!(),
            ASTNode::IfStatement {
                test,
                body,
                alternate,
            } => todo!(),
            ASTNode::MatchStatement { test, body } => todo!(),
            ASTNode::BlockStatement { body } => todo!(),
            ASTNode::ReturnStatement {
                argument,
                is_shortcut,
            } => todo!(),
        }
    }
}
