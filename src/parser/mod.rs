// Allow Parser import, without warning, since it will be generated by a macro.
#[allow(unused_imports)]
use pest::Parser;
use pest::iterators::{Pairs};

// ensure grammar refreshes on compile
const _GRAMMAR: &str = include_str!("grammar.pest");

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct SICParser;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Blur(u32),
    FlipHorizontal,
    FlipVertical,
    Resize(u32, u32),
}

// TODO: proper unwrap() handling
pub fn parse_image_operations(pairs: Pairs<Rule>) -> Vec<Operation> {
    pairs.map(|pair| {
        match pair.as_rule() {
            Rule::blur => {
                let extract_num = pair.into_inner().next().unwrap().as_str().parse::<u32>().unwrap();
                Operation::Blur(extract_num)
            },
            Rule::flip_horizontal => Operation::FlipHorizontal,
            Rule::flip_vertical => Operation::FlipVertical,
            Rule::resize => {
                let inner = pair.into_inner();
                let extract_x = inner.clone().nth(0).unwrap().as_str().parse::<u32>().unwrap();
                let extract_y = inner.clone().nth(1).unwrap().as_str().parse::<u32>().unwrap();
                Operation::Resize(extract_x, extract_y)
            }
            _ => unreachable!(),
        }
    }).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blur_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "blur 15;").unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(vec![Operation::Blur(15)], parse_image_operations(pairs));
    }

    #[test]
    fn test_flip_horizontal_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "flip_horizontal;").unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(vec![Operation::FlipHorizontal], parse_image_operations(pairs));
    }


    #[test]
    fn test_flip_vertical_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "flip_vertical;").unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(vec![Operation::FlipVertical], parse_image_operations(pairs));
    }

    #[test]
    fn test_resize_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "resize 99 88;").unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(vec![Operation::Resize(99, 88)], parse_image_operations(pairs));
    }

    #[test]
    fn test_multi_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "blur 10;flip_horizontal;flip_vertical;resize 100 200;").unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(vec![Operation::Blur(10), Operation::FlipHorizontal, Operation::FlipVertical, Operation::Resize(100, 200)], parse_image_operations(pairs));
    }

    #[test]
    fn test_multi_stmt_parse_diff_order_correct() {
        let pairs = SICParser::parse(Rule::main, "flip_horizontal;flip_vertical;resize 100 200;blur 10;").unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(vec![Operation::FlipHorizontal, Operation::FlipVertical, Operation::Resize(100, 200), Operation::Blur(10)], parse_image_operations(pairs));
    }

    #[test]
    fn test_multi_whitespace() {
        let pairs = SICParser::parse(Rule::main, "flip_horizontal; flip_vertical; resize 100 200; blur 10;").unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(vec![Operation::FlipHorizontal, Operation::FlipVertical, Operation::Resize(100, 200), Operation::Blur(10)], parse_image_operations(pairs));
    }

    #[test]
    fn test_multi_whitespace_2() {
        let pairs = SICParser::parse(Rule::main, "flip_horizontal    ; flip_vertical   ;   \t\t resize 100 200; blur 10;").unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(vec![Operation::FlipHorizontal, Operation::FlipVertical, Operation::Resize(100, 200), Operation::Blur(10)], parse_image_operations(pairs));
    }

    #[test]
    fn test_multi_whitespace_3() {
        let pairs = SICParser::parse(Rule::main, "flip_horizontal;\nflip_vertical;\nresize 100 200;\n\tblur 10;").unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(vec![Operation::FlipHorizontal, Operation::FlipVertical, Operation::Resize(100, 200), Operation::Blur(10)], parse_image_operations(pairs));
    }

    #[test]
    #[should_panic]
    fn test_multi_should_end_with_sep() {
        let pairs = SICParser::parse(Rule::main, "flip_horizontal; flip_vertical; resize 100 200; blur 10").unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(vec![Operation::FlipHorizontal, Operation::FlipVertical, Operation::Resize(100, 200), Operation::Blur(10)], parse_image_operations(pairs));
    }
}