use crate::scanner::Token;

struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
