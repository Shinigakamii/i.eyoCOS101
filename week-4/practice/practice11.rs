fn main() {
	Let a:i32 = 2; 		// Bit presentation 10
	Let b:i32 = 3; 		// Bit presentation 11

Let mut result: i32;

result = a & b;
println! ("(a & b) => {}", result);

result = a | b;
println!("(a | b) => {}", result);

result = a ^ b;
println! ("(a ^ b) => {}", result);

result = !b;
println!("(!b) => {}", result);

result = a << b;
println!("(a << b) => {}", result);

result = a >> b;
println!("(a >> b) => {}", result);
}