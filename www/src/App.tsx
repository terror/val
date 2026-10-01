import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from '@/components/ui/resizable';
import type { Range } from '@/lib/types';
import type { EditorState } from '@codemirror/state';
import type { EditorView, ViewUpdate } from '@codemirror/view';
import { Loader2 } from 'lucide-react';
import { useCallback, useEffect, useState } from 'react';

import { AstPane } from './components/ast-pane';
import { EditorPane } from './components/editor-pane';
import { Header } from './components/header';
import { StatusBar } from './components/status-bar';
import { useEditorExtensions } from './hooks/use-editor-extensions';
import { useMediaQuery } from './hooks/use-media-query';
import { usePersistedDoc } from './hooks/use-persisted-doc';
import { useTheme } from './hooks/use-theme';
import { useValAst } from './hooks/use-val-ast';
import { useValWasm } from './hooks/use-val-wasm';
import { examples } from './lib/examples';

const STORAGE_KEY_CODE = 'val-editor-code';
const STORAGE_KEY_EXAMPLE = 'val-editor-example';
const PANEL_LAYOUT_STORAGE_KEY = 'val-panel-layout';
const DEFAULT_EXAMPLE = 'factorial';
const STACKED_LAYOUT_QUERY = '(max-width: 767px)';

function App() {
  const theme = useTheme();

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

  const { root, errors, collapsedNodes, toggleExpand } = useValAst({
    code,
    loaded,
  });

  const [highlight, setHighlight] = useState<Range | undefined>(undefined);
  const [editorState, setEditorState] = useState<EditorState>();

  const stackedLayout = useMediaQuery(STACKED_LAYOUT_QUERY);
  const panelDirection = stackedLayout ? 'vertical' : 'horizontal';

  const extensions = useEditorExtensions({
    darkMode: theme.darkMode,
    errors,
    highlight,
  });

  const handleHighlightChange = useCallback((range: Range | undefined) => {
    setHighlight(range);
  }, []);

  const handleCreateEditor = useCallback((view: EditorView) => {
    setEditorState(view.state);
  }, []);

  const handleEditorUpdate = useCallback((update: ViewUpdate) => {
    if (update.docChanged || update.selectionSet) {
      setEditorState(update.state);
    }
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
      <div className='flex h-dvh items-center justify-center' role='status'>
        <Loader2 className='text-muted-foreground h-8 w-8 animate-spin' />
        <span className='sr-only'>Loading playground</span>
      </div>
    );
  }

  return (
    <div className='flex h-dvh max-w-full flex-col'>
      <Header darkMode={theme.darkMode} onToggleTheme={theme.toggleTheme} />

      <main className='min-h-0 flex-1 overflow-hidden p-4'>
        <div className='flex h-full flex-col overflow-hidden rounded border'>
          <ResizablePanelGroup
            key={panelDirection}
            autoSaveId={`${PANEL_LAYOUT_STORAGE_KEY}:${panelDirection}`}
            direction={panelDirection}
            className='min-h-0 flex-1'
          >
            <ResizablePanel id='editor-panel' defaultSize={50} minSize={30}>
              <EditorPane
                value={code}
                onChange={setCode}
                currentExample={currentExample}
                examples={examples}
                onExampleChange={handleExampleChange}
                onCreateEditor={handleCreateEditor}
                onUpdate={handleEditorUpdate}
                extensions={extensions}
              />
            </ResizablePanel>

            <ResizableHandle />

            <ResizablePanel id='ast-panel' defaultSize={50} minSize={30}>
              <AstPane
                root={root}
                collapsedNodes={collapsedNodes}
                toggleExpand={toggleExpand}
                onHighlightChange={handleHighlightChange}
              />
            </ResizablePanel>
          </ResizablePanelGroup>
          <StatusBar errors={errors} state={editorState} />
        </div>
      </main>
    </div>
  );
}

export default App;
