// Write the body of the above functions. You can use the tests at the end of file to validate your code.

/// A struct to represent a point in 2D space
///
/// Usage example
/// ```
/// assert_eq!(rust_ex::struct_enum::Point2D { x: 3, y: 4 }.x, 3);
/// assert_eq!(rust_ex::struct_enum::Point2D { x: 3, y: 4 }.y, 4);
/// ```
/// Be careful about the visibility of the fields of the struct
pub struct Point2D {
    // Write your code here
    pub x: i32,
    pub y: i32,
}

/// Implement a method to calculate the distance with another point
///
/// Usage example
/// ```
/// assert_eq!(rust_ex::struct_enum::Point2D { x: 3, y: 4 }.distance_with(&rust_ex::struct_enum::Point2D { x: 0, y: 0 }), 5.0);
/// assert_eq!(rust_ex::struct_enum::Point2D { x: 3, y: 4 }.distance_with(&rust_ex::struct_enum::Point2D { x: 3, y: 4 }), 0.0);
/// assert_eq!(rust_ex::struct_enum::Point2D { x: 3, y: 4 }.distance_with(&rust_ex::struct_enum::Point2D { x: -2, y: 4 }), 5.0);
/// ```
impl Point2D {
    pub fn distance_with(&self, other: &Point2D) -> f32 {
        // Write your code here
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
        ((x_diff.pow(2) + y_diff.pow(2)) as f32).sqrt()
    }
}

/// An enum to represent a shape
/// The shape can be a circle or a rectangle
/// The circle is defined by its center and its radius
/// The rectangle is defined by its top left corner and its bottom right corner
/// ```
/// let c = rust_ex::struct_enum::Shape::Circle { center: rust_ex::struct_enum::Point2D { x: 3, y: 4 }, radius: 5.0 };
/// let r = rust_ex::struct_enum::Shape::Rectangle { top_left: rust_ex::struct_enum::Point2D { x: -3, y: -4 }, bottom_right: rust_ex::struct_enum::Point2D { x: 5, y: 6 } };
/// match c {
///     rust_ex::struct_enum::Shape::Circle { center, radius } => {
///         assert_eq!(center.x, 3);
///         assert_eq!(center.y, 4);
///         assert_eq!(radius, 5.0);
///    },
///    _ => panic!()
/// };
/// match r {
///     rust_ex::struct_enum::Shape::Rectangle { top_left, bottom_right } => {
///         assert_eq!(top_left.x, -3);
///         assert_eq!(top_left.y, -4);
///         assert_eq!(bottom_right.x, 5);
///         assert_eq!(bottom_right.y, 6);
///     },
///     _ => panic!()
/// };
/// ```
pub enum Shape {
    Circle {
        // Write your code here
        center: Point2D,
        radius: f32,
    },
    Rectangle {
        // Write your code here
        top_left: Point2D,
        bottom_right: Point2D,
    },
}

/// Implement a method to make a symetric shape with symetry axis x
///
/// Usage example
/// ```
/// let c = rust_ex::struct_enum::Shape::Circle { center: rust_ex::struct_enum::Point2D { x: 3, y: 4 }, radius: 5.0 }.symetric_x();
/// let r = rust_ex::struct_enum::Shape::Rectangle { top_left: rust_ex::struct_enum::Point2D { x: -3, y: -4 }, bottom_right: rust_ex::struct_enum::Point2D { x: 5, y: 6 } }.symetric_x();
/// match c {
///     rust_ex::struct_enum::Shape::Circle { center, radius } => {
///         assert_eq!(center.x, -3);
///         assert_eq!(center.y, 4);
///         assert_eq!(radius, 5.0);
///     },
///     _ => panic!()
/// };
/// match r {
///     rust_ex::struct_enum::Shape::Rectangle { top_left, bottom_right } => {
///         assert_eq!(top_left.x, 3);
///         assert_eq!(top_left.y, -4);
///         assert_eq!(bottom_right.x, -5);
///         assert_eq!(bottom_right.y, 6);
///     },
///     _ => panic!()
/// };
/// ```
impl Shape {
    pub fn symetric_x(&self) -> Shape {
        // Write your code here
        match self {
            Shape::Circle { center, radius } => Shape::Circle {
                center: Point2D {
                    x: -center.x,
                    y: center.y,
                },
                radius: *radius,
            },
            Shape::Rectangle {
                top_left,
                bottom_right,
            } => Shape::Rectangle {
                top_left: Point2D {
                    x: -top_left.x,
                    y: top_left.y,
                },
                bottom_right: Point2D {
                    x: -bottom_right.x,
                    y: bottom_right.y,
                },
            },
        }
    }
}

// Don't change the following code
#[derive(Debug, PartialEq)]
pub enum TokenType {
    String,
    Number,
    Operator,
    Parenthesis,
    SemiColon,
}
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

// description of the tokens:
// - String: anything between 2 double quotes
// - Number: a sequence of digits and a possible dot
// - Operator: +, -, *, /
// - Parenthesis: (, )
// - SemiColon: ;
/// Tokenize a string into a vector of tokens
/// For the sake of simplicity every value is stored as a string
///
/// Usage example
/// ```
/// let tokens = rust_ex::struct_enum::tokenize("\"hello\" + 39;".to_string());
/// assert_eq!(tokens.len(), 4);
/// assert_eq!(tokens[0].token_type, rust_ex::struct_enum::TokenType::String);
/// assert_eq!(tokens[0].value, "hello");
/// assert_eq!(tokens[1].token_type, rust_ex::struct_enum::TokenType::Operator);
/// assert_eq!(tokens[1].value, "+");
/// assert_eq!(tokens[2].token_type, rust_ex::struct_enum::TokenType::Number);
/// assert_eq!(tokens[2].value, "39");
/// assert_eq!(tokens[3].token_type, rust_ex::struct_enum::TokenType::SemiColon);
/// let tokens = rust_ex::struct_enum::tokenize("\"Salut\" - (3.14);".to_string());
/// assert_eq!(tokens.len(), 6);
/// assert_eq!(tokens[0].token_type, rust_ex::struct_enum::TokenType::String);
/// assert_eq!(tokens[0].value, "Salut");
/// assert_eq!(tokens[1].token_type, rust_ex::struct_enum::TokenType::Operator);
/// assert_eq!(tokens[1].value, "-");
/// assert_eq!(tokens[2].token_type, rust_ex::struct_enum::TokenType::Parenthesis);
/// assert_eq!(tokens[2].value, "(");
/// assert_eq!(tokens[3].token_type, rust_ex::struct_enum::TokenType::Number);
/// assert_eq!(tokens[3].value, "3.14");
/// assert_eq!(tokens[4].token_type, rust_ex::struct_enum::TokenType::Parenthesis);
/// assert_eq!(tokens[4].value, ")");
/// assert_eq!(tokens[5].token_type, rust_ex::struct_enum::TokenType::SemiColon);
/// ```
/// The function should panic if it encounters a token that it does not recognize (except for whitespaces)
/// ```rust,should_panic
/// rust_ex::struct_enum::tokenize("4 § 5".to_string());
/// ```
pub fn tokenize(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    // Write your code here
    let mut i = 0;
    while i < input.len() {
        let mut current_token = String::new();
        let c = input.chars().nth(i).unwrap();
        tokens.push(match c {
            '"' => {
                i += 1;
                while input.chars().nth(i).unwrap() != '"' {
                    current_token.push(input.chars().nth(i).unwrap());
                    i += 1;
                }
                Token {
                    token_type: TokenType::String,
                    value: current_token,
                }
            }
            '0'..='9' => {
                while input.chars().nth(i).unwrap().is_ascii_digit()
                    || input.chars().nth(i).unwrap() == '.'
                {
                    current_token.push(input.chars().nth(i).unwrap());
                    i += 1;
                }
                i -= 1;
                Token {
                    token_type: TokenType::Number,
                    value: current_token,
                }
            }
            '(' | ')' => Token {
                token_type: TokenType::Parenthesis,
                value: c.to_string(),
            },
            '+' | '-' | '*' | '/' => Token {
                token_type: TokenType::Operator,
                value: c.to_string(),
            },
            ';' => Token {
                token_type: TokenType::SemiColon,
                value: c.to_string(),
            },
            ' ' => {
                i += 1;
                continue;
            }
            _ => {
                panic!("Invalid character: {}", c);
            }
        });
        i += 1;
    }
    tokens
}