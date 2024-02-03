pub struct TokenizedProgram {}

pub fn tokenizer(input: &String) -> &String {
    let bytes = input.as_bytes();
    let mut token_start = 0;
    for (i, &item) in bytes.iter().enumerate() {
        println!("input[{}] = {}", i, &input[i..i + 1]);
        if item == b'(' || item == b')' {
            let token = &input[token_start..token_start + 1];
            token_start = i;
        }
        if item == b' ' {
            let token = &input[token_start..i];
            token_start = i;
        }
    }
    let last_token = &input[token_start..];

    return input;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 1 + 1;
        assert_eq!(result, 2);
    }
}
