use crate::parser::*;
use crate::lexer::*;
use llvm_sys::*;

use crate::codegen::stack::*;
use crate::codegen::object::*;

use crate::parser::ast::*;

use crate::ir::block::*;
use crate::ir::condition::*;
use crate::ir::const_value::*;
use crate::ir::converter::*;
use crate::ir::creator::*;
use crate::ir::function::*;
use crate::ir::llvm_type::*;
use crate::ir::operate::*;
use crate::ir::test_util::*;
use crate::ir::validate::*;


pub struct Eval {
    pub stack_arg: Vec<Vec<Expr>>,
    pub lc: LLVMCreator,
    pub main_block: *mut LLVMBasicBlock,
    pub function_stack: FunctionStack,
}

#[allow(dead_code)]
impl Eval {

    pub fn new() -> Self {
        let mut lc = LLVMCreator::new("main_module");
        let (main_block, main_function) = Eval::setup_main(&mut lc);

        Eval {
            stack_arg: Vec::new(),
            lc: lc,
            main_block: main_block,
            function_stack: FunctionStack::new(main_function),
        }
    }

    pub fn entry_eval_program(&mut self, program: Program, env: &mut Environment) -> Object {
        for statement in program.into_iter() {
            if let Some(mut obj) = self.eval_statement(statement, env) {
                let llvm_value = unwrap_object(&mut obj);
                build_ret(self.lc.builder, llvm_value);
                return obj;
            }
        }
        build_ret(self.lc.builder, const_int(int32_type(), 0));
        Object::Null
    }

    pub fn eval_program(&mut self, program: Program, env: &mut Environment) -> Object {
        for statement in program.into_iter() {
            if let Some(mut obj) = self.eval_statement(statement, env) {
                let llvm_Value = unwrap_object(&mut obj);
                build_ret(self.lc.builder, llvm_value);
                return obj;
            }
        }
        Object::Null
    }

    pub fn eval_statement(
        &mut self,
        statement: Stmt,
        env: &mut Environment,
    ) -> Option<Object> {
        match statement {
            Stmt::Assignment(ident, expr) => {
                let obj = self.eval_assignment_statement(ident, expr, llvm_type, env);
                None
            }
            Stmt::Let(ident, expr, llvm_type) => {
                let obj = self.eval_let_statement(ident, expr, llvm_type, env);
                None
            }
            Stmt::Return(expr) => self.eval_return_statement(expr, env),
            Stmt::Expr(expr) => self.eval_expression_statement(expr, env),
        }
    }


    pub fn eval_assignment_statement(&mut self, ident: Ident, expr: Expr, llvm_type: LLVMExpressionType, env: &mut Environment){
        let identify_object = env.get(&ident.0);
        let llvm_value_ref = match identify_object {
            Object::Integer(reference) => reference,
            Object::Boolean(reference) => reference,
            _ => 0 as *mut LLVMValue,
        };

        let mut object = self.eval_expression(expr, &mut env.clone());
        let llvm_value = unwrap_object(&mut object);
        build_store(self.lc.builder, llvm_value, llvm_value_ref);

        Object::Null
    }

    pub fn eval_let_statement(&mut self, ident: ident, expr_type: LLVMExpressionType, expr: Expr, env: &mut Environment) -> Object {
        let mut object = self.eval_expression(expr, env);

        match expr_type {
            LLVMExpressionType::Call => match object {
                Object::Integer(value) | Object::Boolean(value) => {
                    self.set_value_to_identify(value, object, &ident.0, env);
                },
                _ => env.set(ident.0, object),
            }
        }
        _ => self.set_value_to_identify(llvm_value, object, &ident.0, env),
    }

    pub fn eval_expression_statement(&mut self, expr: Expr, env: &mut Environment) -> Option<Object> {
        match expr {
            Expression::If {
                cond,
                consequence,
                alternative
            } => {
                let object = self.eval_if(cond, consequence, alternative);
                None
            }
            _ => {
                let obj = self.eval_expression(expr, env);
                None
            }

        }
    }

    pub fn parse_return_statement(
        &mut self,
        expr: Expr,
        env: &mut Environment
    ){
        self.eval_expression(expr, env);
    }

    pub fn eval_if(
        &mut self,
        cond: Box<Expr>,
        consequence: Program,
        alternative: Option<Program>,
        env: &mut Environment,
    ) -> Option<Object> {
        let current_function = self.function_stack.last();
        let mut return_obj = Object::Null;

        let boolean: *mut LLVMValue = unwrap_object(self.eval_expression(cond, &mut env.clone()));

        left_block = append_basic_block_in_context(
            self.lc.context,
            current_function,
            "",
        );

        let right_block = append_basic_block_in_context(
            self.lc.context,
            current_function,
            "",
        );

        let end_block = append_basic_block_in_context(self.lc.context, current_function, "");

        build_cond_br(self.lc.builder, boolean, left_block, right_block);
        build_position_at_end(self.lc.builder, left_block);
        return_obj = self.eval_program(consequence.clone(), env);
        build_br(self.lc.builder, end_block);

        build_position_at_end(self.lc.builder, right_block);
        return_obj = self.eval_program(alternative.clone(), env);

        build_br(self.lc.builder, end_block);
        build_position_at_end(self.lc.builder, end_block);

        match return_obj {
            Object::Null => None,
            _ => Some(return_obj),
        }
    }

    pub fn eval_expression(&mut self, expr: Expr, env: &mut Environment) {
        match expr {
            Expr::Literal(literal) => {
                match literal {
                    Int(value) => Object::Integer(const_int(int32_type(), value)),
                    Bool(value) => Object::Boolean(const_int(int1_type(), value)),
                }
            }
        }
    }

    pub fn set_value_to_identify(
        &mut self,
        llvm_value: &mut LLVMValue,
        mut object: Object,
        name: &str,
        env: &mut Environment,
    ) -> {
        let llvm_type = get_llvm_type_from_object(&mut object);
        let llvm_value_ref = build_alloc(self.lc.builder, llvm_type, name);
        build_store(self.lc.builder, llvm_value, llvm_value_ref);
        let rewraped_object = rewrap_llvm_value_ref(object, llvm_value_ref);
        env.set(name.to_string(), rewraped_object);
    }
}


