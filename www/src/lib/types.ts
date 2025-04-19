export type AstNode = {
  kind: string;
  range: Range;
  children: AstNode[];
}

export type Range = {
  start: number;
  end: number;
}

export type ValErrorKind = 'Parser' | 'Evaluator';

export type ValError = {
  kind: ValErrorKind;
  message: string;
  range: Range;
}
