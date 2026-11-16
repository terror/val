use {super::*, chumsky::input::WithContext};

type LexerError<'a> = extra::Err<Rich<'a, char, Span>>;
type LexerInput<'a> = WithContext<Span, &'a str>;

pub(crate) const KEYWORDS: [&str; 13] = [
  "break", "continue", "else", "false", "fn", "for", "if", "in", "loop",
  "null", "return", "true", "while",
];

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Token<'a> {
  Comment(&'a str),
  Identifier(&'a str),
  Keyword(&'a str),
  Number(&'a str),
  String(&'a str),
  Symbol(&'a str),
}

impl<'a> Token<'a> {
  pub(crate) fn text(&self) -> &'a str {
    match self {
      Self::Comment(text)
      | Self::Identifier(text)
      | Self::Keyword(text)
      | Self::Number(text)
      | Self::String(text)
      | Self::Symbol(text) => text,
    }
  }
}

impl Display for Token<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_str(self.text())
  }
}

pub(crate) fn lex(
  source: &Source,
) -> Result<Vec<(Token<'_>, Span)>, Vec<Error>> {
  lexer()
    .parse(source.text().with_context::<Span>(source.clone()))
    .into_result()
    .map_err(|errors| {
      errors
        .into_iter()
        .map(|error| Error::new(error.span().clone(), error.to_string()))
        .collect()
    })
}

fn lexer<'a>()
-> impl Parser<'a, LexerInput<'a>, Vec<(Token<'a>, Span)>, LexerError<'a>> {
  let comment = just("//")
    .then(none_of('\n').repeated())
    .to_slice()
    .map(Token::Comment);

  let number = text::int(10)
    .then(just('.').then(text::digits(10)).or_not())
    .then(
      one_of("eE")
        .then(one_of("+-").or_not())
        .then(text::digits(10))
        .or_not(),
    )
    .to_slice()
    .then(
      any()
        .filter(text::Char::is_ident_continue)
        .repeated()
        .to_slice(),
    )
    .validate(|(number, suffix): (&str, &str), extra, emitter| {
      if !suffix.is_empty() {
        emitter.emit(Rich::custom(
          extra.span(),
          format!("invalid number `{number}{suffix}`"),
        ));
      }

      Token::Number(number)
    });

  let word = text::ident().map(|word| {
    if KEYWORDS.contains(&word) {
      Token::Keyword(word)
    } else {
      Token::Identifier(word)
    }
  });

  let symbol = choice((
    just(">="),
    just("<="),
    just("=="),
    just("!="),
    just("&&"),
    just("||"),
  ))
  .or(one_of("()[]{},;+-*/%^<>=!").to_slice())
  .map(Token::Symbol);

  let unexpected = any()
    .filter(|character: &char| {
      !character.is_ascii_digit()
        && !text::Char::is_ident_start(character)
        && !"\"'()[]{},;+-*/%^<>=!".contains(*character)
    })
    .try_map(|character, span| {
      Err(Rich::custom(
        span,
        format!("unexpected character `{character}`"),
      ))
    });

  let token = choice((
    comment,
    number,
    string_parser('"'),
    string_parser('\''),
    word,
    symbol,
    unexpected,
  ))
  .map_with(|token, extra| (token, extra.span()));

  text::whitespace().ignore_then(
    token
      .then_ignore(text::whitespace())
      .repeated()
      .collect::<Vec<_>>(),
  )
}

fn string_parser<'a>(
  quote: char,
) -> impl Parser<'a, LexerInput<'a>, Token<'a>, LexerError<'a>> + Clone {
  just(quote)
    .then(none_of(quote).repeated())
    .then(just(quote).or_not())
    .to_slice()
    .validate(move |string: &str, extra, emitter| {
      if string.len() == 1 || !string.ends_with(quote) {
        emitter.emit(Rich::custom(extra.span(), "unterminated string"));
      }

      Token::String(string)
    })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn errors() {
    #[track_caller]
    fn case(text: &str, range: Range<usize>, message: &str) {
      let source = Source::from(text);

      assert_eq!(
        lex(&source).unwrap_err(),
        [Error::new(source.span(range), message)],
      );
    }

    case("1foo", 0..4, "invalid number `1foo`");
    case("01", 0..2, "invalid number `01`");
    case("1e", 0..2, "invalid number `1e`");
    case("foo @", 4..5, "unexpected character `@`");
    case("'foo", 0..4, "unterminated string");
  }

  #[test]
  fn tokens() {
    let source = Source::from(" foo(1.5, 'bar') >= // baz\nwhile");

    assert_eq!(
      lex(&source)
        .unwrap()
        .into_iter()
        .map(|(token, span)| (token, span.range()))
        .collect::<Vec<_>>(),
      [
        (Token::Identifier("foo"), 1..4),
        (Token::Symbol("("), 4..5),
        (Token::Number("1.5"), 5..8),
        (Token::Symbol(","), 8..9),
        (Token::String("'bar'"), 10..15),
        (Token::Symbol(")"), 15..16),
        (Token::Symbol(">="), 17..19),
        (Token::Comment("// baz"), 20..26),
        (Token::Keyword("while"), 27..32),
      ],
    );
  }
}
