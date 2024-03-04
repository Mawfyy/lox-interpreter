use crate::scanner::Token;

struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(mut self, tokens: Vec<Token>) {
        self.tokens = tokens;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
