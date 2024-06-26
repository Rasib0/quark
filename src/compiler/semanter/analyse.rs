use anyhow::Result;

use crate::compiler::parser::parse::Tree;

impl Tree
{
	/// Semantically analyses the abstract syntax tree.
	///
	/// ### Returns
	/// * The abstract syntax tree if it is semantically accurate.
	///
	/// ### Errors
	/// * If there are semantic errors in the abstract syntax tree.
	pub fn analyse(self, _source: &[Vec<char>]) -> Result<Self>
	{
		let Self(programme) = &self;
		programme.analyse()?;
		Ok(self)
	}
}
