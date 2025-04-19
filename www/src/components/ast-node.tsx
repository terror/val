import { addHighlightEffect, removeHighlightEffect } from '@/lib/cm-highlight-extension';
import { ChevronDown, ChevronRight } from 'lucide-react';
import React, { useState, useCallback, memo } from 'react';
import { AstNode as AstNodeType } from '@/lib/types';
import { EditorView } from '@codemirror/view';

interface AstNodeProps {
  node: AstNodeType;
  depth?: number;
  editorView?: EditorView | null;
}

export const AstNode: React.FC<AstNodeProps> = memo(({ node, depth = 0, editorView }) => {
  const [isExpanded, setIsExpanded] = useState(true);
  const [isHovered, setIsHovered] = useState(false);

  const hasChildren = node.children && node.children.length > 0;

  const isValidRange = node.range.start < node.range.end;

  const style = {
    backgroundColor: isHovered ? 'rgba(59, 130, 246, 0.1)' : 'transparent',
    borderRadius: '2px',
    paddingLeft: `${depth * 16}px`,
  };

  const handleMouseOver = useCallback(() => {
    setIsHovered(true);

    if (editorView && isValidRange) {
      editorView.dispatch({
        effects: addHighlightEffect.of({
          from: node.range.start,
          to: node.range.end
        })
      });
    }
  }, [editorView, node.range.start, node.range.end, isValidRange]);

  const handleMouseLeave = useCallback(() => {
    setIsHovered(false);

    if (editorView && isValidRange) {
      editorView.dispatch({
        effects: removeHighlightEffect.of(null)
      });
    }
  }, [editorView, isValidRange]);

  const toggleExpanded = useCallback(() => {
    setIsExpanded(prev => !prev);
  }, []);

  return (
    <div>
      <div
        className='flex cursor-pointer items-center py-1 font-mono text-sm whitespace-nowrap transition-colors hover:bg-blue-50'
        onClick={toggleExpanded}
        onMouseLeave={handleMouseLeave}
        onMouseOver={handleMouseOver}
        style={style}
      >
        <span className='mr-1 flex w-4 justify-center'>
          {hasChildren ? (
            isExpanded ? (
              <ChevronDown size={14} />
            ) : (
              <ChevronRight size={14} />
            )
          ) : (
            <span className='w-4'></span>
          )}
        </span>

        <span>{node.kind}</span>

        <span className='ml-2 text-xs text-gray-500'>
          [{node.range.start}: {node.range.end}]{!isValidRange && ' (empty)'}
        </span>
      </div>

      {isExpanded && hasChildren && (
        <div className='ml-2'>
          {node.children.map((child, index) => (
            <AstNode
              key={`${child.kind}-${index}`}
              node={child}
              depth={depth + 1}
              editorView={editorView}
            />
          ))}
        </div>
      )}
    </div>
  );
});

AstNode.displayName = 'AstNode';
