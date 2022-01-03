use std::{
    collections::{
        LinkedList, 
        HashMap
    }, 
};

use simple_pascal_ast::{
    node::*, 
    token::{
        literal::Literal, 
        keyword::Keyword, op::{OpKind, Fixity}
    }
};

#[derive(Debug)]
pub enum InterpreterErr {
    InvalidLiteral,
    InvalidUnaryOp,
    InvalidBinOp,
    UndefinedIdent,
    InvalidAssignment,
    UndefinedErr,
}

#[derive(Debug, Default)]
pub struct Interpreter {
    vars: LinkedList<HashMap<String, f64>>,
    hist_vars: LinkedList<HashMap<String, f64>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn interpret(&mut self, ast: Node) -> Result<LinkedList<HashMap<String, f64>>, InterpreterErr> {
        self.visit(ast)?;

        let mut vars = LinkedList::<HashMap<String, f64>>::default();
        std::mem::swap(&mut vars, &mut self.hist_vars);
        Ok(vars)
    }

    fn visit(&mut self, node: Node) -> Result<f64, InterpreterErr> {
        match node {
            Node::BinOp(bin_op) => self.visit_bin_op(bin_op),
            Node::UnaryOp(unary_op) => self.visit_unary_op(unary_op),
            Node::Literal(lit) => self.visit_literal(lit),
            Node::Ident(ident) => self.visit_ident(&ident),
            Node::Keyword(keyword) => self.visit_keyword(keyword),
            Node::Compound(compound) => {
                self.vars.push_front(HashMap::new());
                let mut res = 0.0;

                for node in compound.children {
                    res = self.visit(node)?;
                }

                let vars = self.vars.pop_front();
                self.log_vars(vars);

                Ok(res)
            },
            Node::None => Ok(0.0),
            _ => Err(InterpreterErr::UndefinedErr)
        }
    }

    fn visit_bin_op(&mut self, bin_op: BinOp) -> Result<f64, InterpreterErr> {
        match bin_op.op.fixity() {
            Fixity::Right => {
                match bin_op.op {
                    OpKind::AssignEq => self.assign_var(*bin_op.lhs, *bin_op.rhs),
                    _ => Err(InterpreterErr::InvalidBinOp)
                }
            },
            Fixity::Left => {
                let lhs = self.visit(*bin_op.lhs)?;
                let rhs = self.visit(*bin_op.rhs)?;

                match bin_op.op {
                    OpKind::Caret => Ok(lhs.powf(rhs)),
                    OpKind::Minus => Ok(lhs - rhs),
                    OpKind::Plus => Ok(lhs + rhs),
                    OpKind::Slash => Ok(lhs / rhs),
                    OpKind::Star => Ok(lhs * rhs),
                    _ => Err(InterpreterErr::InvalidBinOp)
                }
            },
            Fixity::None => Err(InterpreterErr::InvalidBinOp)
        }
    }

    fn visit_unary_op(&mut self, unary_op: UnaryOp) -> Result<f64, InterpreterErr> {
        match unary_op.op {
            OpKind::Minus => Ok(-self.visit(*unary_op.node)?),
            OpKind::Plus => self.visit(*unary_op.node),
            _ => Err(InterpreterErr::InvalidUnaryOp)
        }
    }

    fn visit_literal(&self, lit: Literal) -> Result<f64, InterpreterErr> {
        let res: Result<f64, _> = match lit {
            Literal::Float(float) => float.parse(),
            Literal::Integer(int) => int.parse()
        };

        match res {
            Ok(float) => Ok(float),
            Err(_) => Err(InterpreterErr::InvalidLiteral)
        }
    }

    fn visit_ident(&mut self, ident: &str) -> Result<f64, InterpreterErr> {
        match self.find_ident(ident) {
            Some(ident) => Ok(ident.clone()),
            None => Err(InterpreterErr::UndefinedIdent)
        }
    }

    fn visit_keyword(&mut self, _keyword: Keyword) -> Result<f64, InterpreterErr> {
        todo!("NO KEYWORDS YET")
    }

    fn assign_var(&mut self, var: Node, expr: Node) -> Result<f64, InterpreterErr> {
        let ident = match var {
            Node::Ident(ident) => ident,
            _ => return Err(InterpreterErr::InvalidAssignment)
        };

        let expr_res = self.visit(expr)?;

        match self.find_ident(&ident) {
            Some(var) => {
                *var = expr_res;
                Ok(expr_res)
            },
            None => {
                match self.vars.front_mut() {
                    Some(map) => {
                        map.insert(ident, expr_res);
                        Ok(expr_res)
                    },
                    None => Err(InterpreterErr::UndefinedErr)
                }
            }
        }
    }

    fn find_ident(&mut self, ident: &str) -> Option<&mut f64> {
        for vars in self.vars.iter_mut() {
            if vars.contains_key(ident) {
                return vars.get_mut(ident);
            }
        }
        None
    }

    fn log_vars(&mut self, vars: Option<HashMap<String, f64>>) {
        match vars {
            Some(vars) => self.hist_vars.push_back(vars),
            None => ()
        };
    }
}