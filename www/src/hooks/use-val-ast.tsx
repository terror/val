import { useCallback, useEffect, useMemo, useState } from 'react';
import { type AstNode, type ValError, parse } from 'val-wasm';

interface UseValAstOptions {
  code: string;
  loaded: boolean;
}

interface UseValAst {
  collapsedNodes: Set<AstNode>;
  errors: ValError[];
  root: AstNode | undefined;
  toggleExpand: (node: AstNode) => void;
}

export function useValAst({ code, loaded }: UseValAstOptions): UseValAst {
  const { root, errors } = useMemo(() => {
    if (!loaded) {
      return { root: undefined, errors: [] };
    }

    try {
      const result = parse(code);

      return result.status === 'ok'
        ? { root: result.ast, errors: [] }
        : { root: undefined, errors: result.errors };
    } catch (error) {
      return { root: undefined, errors: parseErrors(error) };
    }
  }, [code, loaded]);

  const [collapsedNodes, setCollapsedNodes] = useState<Set<AstNode>>(
    () => new Set()
  );

  useEffect(() => {
    setCollapsedNodes(new Set());
  }, [root]);

  const toggleExpand = useCallback((node: AstNode) => {
    setCollapsedNodes((previous) => {
      const next = new Set(previous);

      if (next.has(node)) {
        next.delete(node);
      } else {
        next.add(node);
      }

      return next;
    });
  }, []);

  return { root, errors, collapsedNodes, toggleExpand };
}

function parseErrors(error: unknown): ValError[] {
  return [
    {
      kind: 'Parser',
      message: error instanceof Error ? error.message : String(error),
      range: {
        start: 0,
        end: 0,
      },
    },
  ];
}
