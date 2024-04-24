use std::iter::Peekable;

use anyhow::{bail, Result};

use super::*;
use crate::compiler::Error;
use crate::language::grammar::statement::{Statement, Kind, Declaration, Expression};
use crate::language::lexicon::token::{Kind::*, Token};
use crate::language::utils::Span;

impl Statement
{
	pub fn try_from_stream<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Option<Statement>>
	where
		I: Iterator<Item = Token>,
	{
		let token = match stream.peek()
		{
			Some(token) => &token.kind,
			None => return Ok(None),
		};

		let kind = match token
		{
			Constant | Variable =>
			{
				let declaration = Declaration::try_from_stream(stream, source)?;
				Kind::Declaration(declaration)
			}

			_ =>
			{
				let expression = Expression::try_from_stream(stream, source)?;
				Kind::Expression(expression)
			}
		};

		let start = match kind
		{
			Kind::Declaration(declaration) => declaration.span.start,
			Kind::Expression(expression) => expression.span.start,
		};

		let end = match stream.next_if(|token| token.kind == Semicolon)
		{
			Some(token) => token.span.end,
			None =>
			{
				let position = match kind
				{
					Kind::Declaration(declaration) => declaration.span.end,
					Kind::Expression(expression) => expression.span.end,
				};

				let span = Span { start: position, end: position };

				bail!(source.error(span, error::SEMICOLON))
			}
		};

		let span = Span { start, end };

		Ok(Some(Statement { kind, span }))
	}
}
