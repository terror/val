import { ChevronDown, ChevronRight } from 'lucide-react';
import React, { useState } from 'react';
import { AstNode as AstNodeType } from 'val-wasm';

interface AstNodeProps {
  node: AstNodeType;
  depth?: number;
}

export const AstNode: React.FC<AstNodeProps> = ({ node, depth = 0 }) => {
  const [isExpanded, setIsExpanded] = useState(true);
  const [isHovered, setIsHovered] = useState(false);

  const hasChildren = node.children && node.children.length > 0;

  const style = {
    backgroundColor: isHovered ? 'rgba(59, 130, 246, 0.1)' : 'transparent',
    borderRadius: '2px',
    paddingLeft: `${depth * 16}px`,
  };

  return (
    <div>
      <div
        className='flex cursor-pointer items-center py-1 font-mono text-sm whitespace-nowrap transition-colors hover:bg-blue-50'
        onClick={() => setIsExpanded(!isExpanded)}
        onMouseLeave={() => setIsHovered(false)}
        onMouseOver={() => setIsHovered(true)}
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
          [{node.range.start}: {node.range.end}]
        </span>
      </div>

      {isExpanded && hasChildren && (
        <div className='ml-2'>
          {node.children.map((child, index) => (
            <AstNode
              key={`${child.kind}-${index}`}
              node={child}
              depth={depth + 1}
            />
          ))}
        </div>
      )}
    </div>
  );
};
