use itertools::Itertools;
use proc_macro;

fn match_it(token: proc_macro::TokenTree) -> Vec<String> {
    let mut v = vec![];
    match token {
        proc_macro::TokenTree::Literal(literal) => {
            v.push(format!("SantaNum({})", literal));
            v
        }
        proc_macro::TokenTree::Punct(punct) => {
            match punct.as_char() {
                '*' => v.push("+".to_string()),
                '+' => v.push("*".to_string()),
                other => v.push(other.to_string()),
            };

            v
        }
        proc_macro::TokenTree::Group(group) => {
            v.push("(".to_string());
            for item in group.stream() {
                let v2 = match_it(item);
                for item2 in v2 {
                    v.push(item2);
                }
            }
            v.push(")".to_string());
            v
        }
        proc_macro::TokenTree::Ident(ident) => v,
    }
}
#[proc_macro]
pub fn santa_replace(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let str = include_str!("../../day-18/input.txt");
    // dbg!(str);
    let exprs = str
        .to_string()
        .lines()
        .map(|v| {
            let token_stream = v.parse::<proc_macro::TokenStream>().unwrap();
            let mut tokens = vec![];
            for token in token_stream.into_iter() {
                for item in match_it(token) {
                    tokens.push(item);
                }
            }
            tokens.into_iter().map(|v| v + " ").collect::<String>()
        })
        .intersperse(",\n".to_string())
        .collect::<String>();
    format!("vec![{}]", exprs).parse().unwrap()

    // token_string.parse().unwrap()
}

//340789638435483
// #[proc_macro]
// pub fn santa_replace(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let mut tokens = vec![];
//     for token in input.into_iter() {
//         for item in match_it(token) {
//             tokens.push(item);
//         }
//     }
//     let token_string = tokens.into_iter().map(|v| v + " ").collect::<String>();

//     token_string.parse().unwrap()
// }
