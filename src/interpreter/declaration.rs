use crate::parser::{assignment::initial::VariableKeyword, ast::vars::VariableDeclarator};

use super::SymbolTable;

    pub fn add_declarations(
        symbol_table: &mut SymbolTable,
        kind: VariableKeyword,
        declarations: Vec<VariableDeclarator>,
    ) -> Result<(), String> {
        match kind {
            VariableKeyword::Var => symbol_table.add_variables_declarations(declarations)?,
            VariableKeyword::Let => symbol_table.add_constants_declarations(declarations)?,
        };

        Ok(())
    }
    fn add_variables_declarations(
        symbol_table: &mut SymbolTable,
        declarations: Vec<VariableDeclarator>,
    ) -> Result<(), String> {
        for declaration in declarations {
            let name = declaration.id.name.to_owned();
            let expr = symbol_table.evaluate_expr(declaration.init)?;

            symbol_table.add_variable(name, expr)
        }

        Ok(())
    }
    fn add_constants_declarations(
        symbol_table: &mut SymbolTable,
        declarations: Vec<VariableDeclarator>,
    ) -> Result<(), String> {
        for declaration in declarations {
            let name = declaration.id.name.to_owned();
            let expr = symbol_table.evaluate_expr(declaration.init)?;

            symbol_table.add_constant(name, expr)
        }

        Ok(())
    }