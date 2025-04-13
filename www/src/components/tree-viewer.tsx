import { AstNode } from '@/lib/types';
import { ChevronDown, ChevronRight } from 'lucide-react';
import React, { useState } from 'react';

interface AttributesProps {
  attributes: Map<string, string>;
  depth: number;
}

const Attributes: React.FC<AttributesProps> = ({ attributes, depth }) => {
  return !attributes || Array.from(attributes.entries()).length === 0 ? null : (
    <>
      {Array.from(attributes.entries()).map(([key, value]) => (
        <div
          key={key}
          className='hover:bg-accent/50 flex items-center rounded-sm'
          style={{ paddingLeft: `${depth * 16}px` }}
        >
          <div className='flex w-6 items-center justify-center' />
          <div className='text-muted-foreground flex-1 px-1 py-1 font-mono text-sm'>
            {key}: {value}
          </div>
        </div>
      ))}
    </>
  );
};

interface TreeNodeProps {
  node: AstNode;
  depth?: number;
  onHover: (node: AstNode) => void;
  clearHighlight: () => void;
}

const TreeNode: React.FC<TreeNodeProps> = ({
  node,
  depth = 0,
  onHover,
  clearHighlight,
}) => {
  const [isExpanded, setIsExpanded] = useState(true);

  const hasChildren = node.children && node.children.length > 0;

  const attributes = node.attributes as any;

  return (
    <div className='cursor-pointer select-none'>
      <div
        className='hover:bg-accent/50 flex items-center rounded-sm'
        onClick={() => setIsExpanded(!isExpanded)}
        onMouseLeave={clearHighlight}
        onMouseOver={() => onHover(node)}
        style={{ paddingLeft: `${depth * 16}px` }}
      >
        <div
          className={`flex cursor-pointer items-center gap-x-1 px-1 py-1 ${depth === 0 ? 'font-semibold' : ''}`}
        >
          {isExpanded ? (
            <ChevronDown className='h-4 w-4' />
          ) : (
            <ChevronRight className='h-4 w-4' />
          )}
          <span className='font-mono text-neutral-800 dark:text-neutral-200'>
            {node.kind}
          </span>
        </div>
      </div>

      {isExpanded && attributes && Array.from(attributes.entries()).length && (
        <Attributes attributes={attributes} depth={depth} />
      )}

      {isExpanded && hasChildren && (
        <div className='ml-2'>
          {node.children.map((child, index) => (
            <TreeNode
              key={`${child.kind}-${index}`}
              node={child}
              depth={depth + 1}
              onHover={onHover}
              clearHighlight={clearHighlight}
            />
          ))}
        </div>
      )}
    </div>
  );
};

interface TreeViewerProps {
  ast: AstNode | null;
  onNodeHover: (node: AstNode) => void;
  clearHighlight: () => void;
}

export const TreeViewer: React.FC<TreeViewerProps> = ({
  ast,
  onNodeHover,
  clearHighlight,
}) => {
  return ast ? (
    <TreeNode
      node={ast}
      onHover={onNodeHover}
      clearHighlight={clearHighlight}
    />
  ) : (
    <div className='text-muted-foreground p-4'>No AST available</div>
  );
};
