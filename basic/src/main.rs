fn main() {
    let my_question = Question {
        question: String::from("Is Rust a good language?"),
        answer: Option::TrueOption,
    };

    println!("Question: {}", my_question.question);
    println!("Answer: {}", my_question.answer.is_true_let());

    let another_question = Question {
        question: String::from("Is Rust a bad language?"),
        answer: Option::FalseOption,
    };
    println!("Question: {}", another_question.question);
    println!("Answer: {}", another_question.answer.is_false_match());
}

enum Option {
    TrueOption,
    FalseOption,
}

struct Question {
    question: String,
    answer: Option,
}

impl Option {
    fn is_true_let(&self) -> bool {
        if let Option::TrueOption = self {
            true
        } else {
            false
        }
    }

    fn is_false_match(&self) -> bool {
        match self {
            Option::FalseOption => false,
            _ => true,
        }
    }
}
