use quark::compiler::Compile;
const HEADER: &str = "import numpy as np\n";

#[test]
fn comments_work()
{
	let input1 = "
let y = 1;

// let y = [1, 2|
/*       |1, 2];
 let x = [1, 2| 1, 2]; */
// let answer = x + y;
let answer = [1, 2];
let monkey = 1 + 2 / 2 / 2;
//print(answer);
"
	.to_string();

	let input2 = "
let y = 1;

let answer = [1, 2];
let monkey = 1 + 2 / 2 / 2;
"
	.to_string();

	let output1 = input1.compile().unwrap();
	let output2 = input2.compile().unwrap();
	assert_eq!(output2, output1);
}

#[test]
fn testing_new_array_expression()
{
	let input = "
let x = [1, 2, 3]a;
"
	.to_string();

	let expected = "x = [1, 2, 3]".to_string();

	let output = input.compile().unwrap();
	assert_eq!(output, format!("{}{}", HEADER, expected));
}

#[test]
fn testing_array_expression()
{
	let input = "
let x = [1, 2, 3];
"
	.to_string();

	let expected = "x = [1, 2, 3]".to_string();

	let output = input.compile().unwrap();
	assert_eq!(output, format!("{}{}", HEADER, expected));
}

#[test]
fn testing_default_array_expression()
{
	let input = "
let x = [];
"
	.to_string();

	let expected = "x = []".to_string();

	let output = input.compile().unwrap();
	assert_eq!(output, format!("{}{}", HEADER, expected));
}

#[test]
fn testing_empty_array_expression()
{
	let input = "
let x = []a;
"
	.to_string();

	let expected = "x = []".to_string();

	let output = input.compile().unwrap();
	assert_eq!(output, format!("{}{}", HEADER, expected));
}

#[test]
fn testing_empty_matrix_expression()
{
	let input = "
let x = []m;
"
	.to_string();

	let expected = "x = np.array([[],])".to_string();

	let output = input.compile().unwrap();
	assert_eq!(output, format!("{}{}", HEADER, expected));
}

#[test]
fn testing_new_matrix_expression()
{
	let input = "
let x = [1, 2, 3]m;
"
	.to_string();

	let expected = "x = np.array([[1, 2, 3],])".to_string();

	let output = input.compile().unwrap();
	assert_eq!(output, format!("{}{}", HEADER, expected));
}

#[test]
fn testing_new_matrix_expression_without_annotation()
{
	let input = "
let x = [1, 2, 3 | 1, 2, 3 || 1, 2, 3];
"
	.to_string();
	let expected = "x = np.array([[1, 2, 3],[1, 2, 3],[1, 2, 3],])".to_string();

	let output = input.compile().unwrap();
	assert_eq!(output, format!("{}{}", HEADER, expected));
}
