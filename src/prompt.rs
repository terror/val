use super::*;

pub(crate) struct Prompt {
  completer: FilenameCompleter,
  hinter: HistoryHinter,
}

impl Prompt {
  pub(crate) fn new() -> Self {
    Self {
      completer: FilenameCompleter::new(),
      hinter: HistoryHinter::new(),
    }
  }
}

impl Completer for Prompt {
  type Candidate = Pair;

  fn complete(
    &self,
    line: &str,
    pos: usize,
    ctx: &Context<'_>,
  ) -> Result<(usize, Vec<Pair>), ReadlineError> {
    self.completer.complete(line, pos, ctx)
  }
}

impl Helper for Prompt {}

impl Hinter for Prompt {
  type Hint = String;

  fn hint(&self, line: &str, a: usize, b: &Context) -> Option<Self::Hint> {
    self.hinter.hint(line, a, b)
  }
}

impl RustylineHighlighter for Prompt {
  fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
    Highlighter::new(line).highlight()
  }

  fn highlight_char(&self, _: &str, _: usize, _: CmdKind) -> bool {
    true
  }

  fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
    Owned(format!("\x1b[90m{hint}\x1b[0m"))
  }
}

impl Validator for Prompt {}
