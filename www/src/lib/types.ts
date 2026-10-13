export type AstNode = {
  kind: string;
  range: Range;
  children: AstNode[];
};

export type Execution = {
  errors: ValError[];
  exitCode: number;
  output: string;
  value?: string;
};

export type ExecutionRequest = {
  code: string;
  input: string;
};

export type ExecutionState =
  | { status: 'idle' }
  | { status: 'running' | 'stopped'; code: string }
  | { status: 'complete'; code: string; result: Execution };

export type Range = {
  start: number;
  end: number;
};

export type ValErrorKind = 'Parser' | 'Evaluator';

export type ValError = {
  kind: ValErrorKind;
  message: string;
  range: Range;
};
