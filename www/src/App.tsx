import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from '@/components/ui/resizable';
import type { Range } from '@/lib/types';
import { Loader2, Radius } from 'lucide-react';
import { useCallback, useEffect, useState } from 'react';

import { AstPane } from './components/ast-pane';
import { EditorPane } from './components/editor-pane';
import { useEditorExtensions } from './hooks/use-editor-extensions';
import { useMediaQuery } from './hooks/use-media-query';
import { usePersistedDoc } from './hooks/use-persisted-doc';
import { useValAst } from './hooks/use-val-ast';
import { useValWasm } from './hooks/use-val-wasm';
import { examples } from './lib/examples';

const STORAGE_KEY_CODE = 'val-editor-code';
const STORAGE_KEY_EXAMPLE = 'val-editor-example';
const PANEL_LAYOUT_STORAGE_KEY = 'val-panel-layout';
const DEFAULT_EXAMPLE = 'factorial';
const STACKED_LAYOUT_QUERY = '(max-width: 767px)';

function App() {
  const [code, setCode] = usePersistedDoc(
    STORAGE_KEY_CODE,
    examples[DEFAULT_EXAMPLE]
  );

  const [currentExample, setCurrentExample] = useState(() => {
    const savedExample = localStorage.getItem(STORAGE_KEY_EXAMPLE);

    return savedExample && savedExample in examples
      ? savedExample
      : DEFAULT_EXAMPLE;
  });

  const { error, loaded, loading } = useValWasm();

  const { root, errors, expandedNodes, toggleExpand } = useValAst({
    code,
    loaded,
  });

  const [highlight, setHighlight] = useState<Range | undefined>(undefined);

  const stackedLayout = useMediaQuery(STACKED_LAYOUT_QUERY);
  const panelDirection = stackedLayout ? 'vertical' : 'horizontal';

  const extensions = useEditorExtensions({
    errors,
    highlight,
  });

  const handleHighlightChange = useCallback((range: Range | undefined) => {
    setHighlight(range);
  }, []);

  useEffect(() => {
    localStorage.setItem(STORAGE_KEY_EXAMPLE, currentExample);
  }, [currentExample]);

  const handleExampleChange = (value: string) => {
    if (!(value in examples)) {
      return;
    }

    setCurrentExample(value);
    setCode(examples[value]);
  };

  if (error) {
    return <div className='p-4'>error: {error}</div>;
  }

  if (loading || !loaded) {
    return (
      <div className='flex h-screen items-center justify-center'>
        <Loader2 className='text-muted-foreground h-8 w-8 animate-spin' />
      </div>
    );
  }

  return (
    <div className='flex h-screen max-w-full flex-col'>
      <div className='flex items-center gap-x-2 px-4 py-4'>
        <Radius className='h-4 w-4' />
        <a href='/val' className='font-semibold'>
          val
        </a>
      </div>

      <div className='flex-1 overflow-hidden p-4 pt-0'>
        <ResizablePanelGroup
          key={panelDirection}
          autoSaveId={`${PANEL_LAYOUT_STORAGE_KEY}:${panelDirection}`}
          direction={panelDirection}
          className='h-full rounded border'
        >
          <ResizablePanel id='editor-panel' defaultSize={50} minSize={30}>
            <EditorPane
              value={code}
              onChange={setCode}
              currentExample={currentExample}
              examples={examples}
              onExampleChange={handleExampleChange}
              extensions={extensions}
            />
          </ResizablePanel>

          <ResizableHandle withHandle />

          <ResizablePanel id='ast-panel' defaultSize={50} minSize={30}>
            <AstPane
              root={root}
              expandedNodes={expandedNodes}
              toggleExpand={toggleExpand}
              onHighlightChange={handleHighlightChange}
            />
          </ResizablePanel>
        </ResizablePanelGroup>
      </div>
    </div>
  );
}

export default App;
