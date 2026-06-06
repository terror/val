import type { AstNode, ValError } from '@/lib/types';
import { useCallback, useEffect, useMemo, useState } from 'react';
import { parse } from 'val-wasm';

interface UseValAstOptions {
  code: string;
  loaded: boolean;
}

interface UseValAst {
  errors: ValError[];
  expandedNodes: Set<AstNode>;
  root: AstNode | undefined;
  toggleExpand: (node: AstNode) => void;
}

export function useValAst({ code, loaded }: UseValAstOptions): UseValAst {
  const { root, errors } = useMemo(() => {
    if (!loaded) {
      return { root: undefined, errors: [] };
    }

    try {
      return { root: parse(code) as AstNode, errors: [] };
    } catch (error) {
      return { root: undefined, errors: parseErrors(error) };
    }
  }, [code, loaded]);

  const [expandedNodes, setExpandedNodes] = useState<Set<AstNode>>(
    () => new Set()
  );

  useEffect(() => {
    if (!root) {
      setExpandedNodes(new Set());
      return;
    }

    const all = new Set<AstNode>();

    const walk = (node: AstNode) => {
      all.add(node);
      node.children.forEach(walk);
    };

    walk(root);

    setExpandedNodes(all);
  }, [root]);

  const toggleExpand = useCallback((node: AstNode) => {
    setExpandedNodes((previous) => {
      const next = new Set(previous);

      if (next.has(node)) {
        next.delete(node);
      } else {
        next.add(node);
      }

      return next;
    });
  }, []);

  return { root, errors, expandedNodes, toggleExpand };
}

function parseErrors(error: unknown): ValError[] {
  if (Array.isArray(error)) {
    return error as ValError[];
  }

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
