# rust-calculator
First try to work with rust. Implement simple calculator for math expressions.

Calculator gets a math expression in infix notation. Expression can include numbers, braces and one of the following operators *, /, +, -. Space characters are not allowed.
Expression is passed as a command line argument for the program. It will be transformed into postfix notation and evaluated.

Run example:

cargo run (13+5)*2

Output example:
You provided following expression: (13+5)*2

Expression in postfix notation 13 5 + 2 *

Result: 36
