import { ChevronDown, ChevronRight } from 'lucide-react';
import { AstNode } from 'val-wasm';
import React, { useState } from 'react';

interface TreeNodeProps {
  node: AstNode;
  depth?: number;
}

const TreeNode: React.FC<TreeNodeProps> = ({ node, depth = 0 }) => {
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
            <TreeNode
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

interface TreeViewerProps {
  ast: AstNode | null;
}

export const TreeViewer: React.FC<TreeViewerProps> = ({ ast }) => {
  return ast ? (
    <TreeNode node={ast} />
  ) : (
    <div className='text-muted-foreground p-4'>No AST available</div>
  );
};
