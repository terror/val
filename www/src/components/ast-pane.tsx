import { AstNode } from '@/components/ast-node';
import { PlaygroundInfoDialog } from '@/components/playground-info-dialog';
import type { AstNode as AstNodeType, Range } from '@/lib/types';

interface AstPaneProps {
  collapsedNodes: Set<AstNodeType>;
  onHighlightChange: (range: Range | undefined) => void;
  root: AstNodeType | undefined;
  toggleExpand: (node: AstNodeType) => void;
}

export const AstPane = ({
  collapsedNodes,
  onHighlightChange,
  root,
  toggleExpand,
}: AstPaneProps) => {
  return (
    <section
      aria-label='Syntax tree'
      className='flex h-full min-h-0 flex-col overflow-hidden'
    >
      <div className='bg-muted/50 flex shrink-0 items-center justify-end gap-1 border-b px-2 py-1'>
        <PlaygroundInfoDialog />
      </div>
      <div className='min-h-0 flex-1 overflow-auto'>
        {root ? (
          <div className='p-2'>
            <AstNode
              node={root}
              level={0}
              collapsedNodes={collapsedNodes}
              toggleExpand={toggleExpand}
              onHighlightChange={onHighlightChange}
            />
          </div>
        ) : (
          <p className='text-muted-foreground p-4 text-center'>
            No AST available
          </p>
        )}
      </div>
    </section>
  );
};
