import { AstNode } from '@/components/ast-node';
import type { AstNode as AstNodeType, Range } from '@/lib/types';

interface AstPaneProps {
  expandedNodes: Set<AstNodeType>;
  onHighlightChange: (range: Range | undefined) => void;
  root: AstNodeType | undefined;
  toggleExpand: (node: AstNodeType) => void;
}

export const AstPane = ({
  expandedNodes,
  onHighlightChange,
  root,
  toggleExpand,
}: AstPaneProps) => {
  return (
    <div className='h-full overflow-auto'>
      {root ? (
        <div className='p-2'>
          <AstNode
            node={root}
            level={0}
            expandedNodes={expandedNodes}
            toggleExpand={toggleExpand}
            onHighlightChange={onHighlightChange}
          />
        </div>
      ) : (
        <p className='p-4 text-center text-gray-500'>No AST available</p>
      )}
    </div>
  );
};
