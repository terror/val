import type { AstNode as AstNodeType, Range } from '@/lib/types';
import { ChevronDown, ChevronRight } from 'lucide-react';
import React, { memo, useCallback } from 'react';

interface AstNodeProps {
  expandedNodes: Set<AstNodeType>;
  level: number;
  node: AstNodeType;
  onHighlightChange: (range: Range | undefined) => void;
  toggleExpand: (node: AstNodeType) => void;
}

export const AstNode: React.FC<AstNodeProps> = memo(
  ({ expandedNodes, level, node, onHighlightChange, toggleExpand }) => {
    const hasChildren = node.children && node.children.length > 0;
    const isExpanded = expandedNodes.has(node);
    const isValidRange = node.range.start < node.range.end;

    const handleMouseEnter = useCallback(() => {
      if (isValidRange) {
        onHighlightChange(node.range);
      }
    }, [isValidRange, node.range, onHighlightChange]);

    const handleMouseLeave = useCallback(() => {
      onHighlightChange(undefined);
    }, [onHighlightChange]);

    const toggleExpanded = useCallback(() => {
      if (hasChildren) {
        toggleExpand(node);
      }
    }, [hasChildren, node, toggleExpand]);

    return (
      <>
        <div
          className='flex cursor-pointer items-center py-1 font-mono text-sm whitespace-nowrap transition-colors hover:bg-blue-50'
          onClick={toggleExpanded}
          onMouseLeave={handleMouseLeave}
          onMouseEnter={handleMouseEnter}
          style={{ paddingLeft: `${level * 16 + 4}px` }}
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

        {isExpanded &&
          hasChildren &&
          node.children.map((child, index) => (
            <AstNode
              key={`${child.kind}-${index}`}
              node={child}
              level={level + 1}
              expandedNodes={expandedNodes}
              toggleExpand={toggleExpand}
              onHighlightChange={onHighlightChange}
            />
          ))}
      </>
    );
  }
);

AstNode.displayName = 'AstNode';
