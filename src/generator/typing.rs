use crate::parser::expression::Expression;
use crate::parser::typing::Typing;
use crate::generator::Generator;

impl Generator {
    pub fn expr_matches_typing(&self, typing: Typing) -> bool {
        // match typing {
        //     Typing::Integer if let Expression:: => true ,
        //     Typing::String => true
        //     Typing::Unknown => true
        // }

        todo!()
    }

    fn get_typing(&self) -> Typing {
        // match self {
        //     Expression::Sum(_, _) => Typing::Unknown,
        //     Expression::Summand(_) => Typing::Unknown,
        //     Expression::Boolean(_) => Typing::Boolean,
        //     Expression::Variable(var) => var.
        // }

        todo!()
    }
}
