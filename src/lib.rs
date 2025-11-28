pub mod arg_util;
pub mod function;

#[cfg(test)]
mod tests {
    use std::iter::Inspect;

    use crate::function::UnaryOperator;

    use super::*;

    #[test]
    fn supplier() {
        let supplier: function::Supplier<String> = | | return "input".to_string();
        assert!(supplier().eq("input"));
    }

    #[test]
    fn consumer() {
        let consumer: function::Consumer<&mut Option<String>> = |x: &mut Option<String> | {
           x.take(); 
        };
        let input = &mut Option::Some("input".to_string());
        consumer(input);
        assert!(input.is_none());
    }

    #[test]
    fn unary_operator() {
        let operation: UnaryOperator<u32> = |x| return x * 2;
        let input = 3;
        let output = operation(input);
        println!("{}", output);
        assert!(output == (input * 2));
    }

}
