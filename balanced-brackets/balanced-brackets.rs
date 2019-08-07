pub fn brackets_are_balanced(string: &str) -> bool {
    fn brackets_match(l: char, r: char) -> bool {
        match (l, r) {
            ('(', ')') => true,
            ('[', ']') => true,
            ('{', '}') => true,
            _ => false,
        }
    }

    let mut stack = Vec::new();

    for r in string.chars() {
        if stack.last().map_or(false, |&l| brackets_match(l, r)) {
            stack.pop();
        } else {
            stack.push(r);
        }
    }

    stack.is_empty()
}

