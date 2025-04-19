import { AstNode } from '@/components/ast-node';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import type { AstNode as AstNodeType, ValError } from '@/lib/types';
import { EditorView } from '@codemirror/view';
import { useEffect, useRef, useState } from 'react';
import { toast } from 'sonner';
import init, { parse } from 'val-wasm';

import { Editor, EditorRef } from './components/editor';
import { EditorSettingsDialog } from './components/editor-settings-dialog';
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from './components/ui/resizable';
import EXAMPLES from './lib/examples';

const STORAGE_KEY_CODE = 'val-editor-code';
const STORAGE_KEY_EXAMPLE = 'val-editor-example';

function App() {
  const [ast, setAst] = useState<AstNodeType | null>(null);

  const [code, setCode] = useState(() => {
    const savedCode = localStorage.getItem(STORAGE_KEY_CODE);
    return savedCode || EXAMPLES.factorial;
  });

  const [currentExample, setCurrentExample] = useState(() => {
    const savedExample = localStorage.getItem(STORAGE_KEY_EXAMPLE);
    return savedExample || 'factorial';
  });

  const [editorView, setEditorView] = useState<EditorView | null>(null);
  const [errors, setErrors] = useState<ValError[]>([]);
  const [wasmLoaded, setWasmLoaded] = useState(false);

  const editorRef = useRef<EditorRef>(null);

  const handleEditorReady = (view: EditorView) => {
    setEditorView(view);
  };

  useEffect(() => {
    init()
      .then(() => {
        setWasmLoaded(true);
      })
      .catch((error) => {
        toast.error(error);
      });
  }, []);

  useEffect(() => {
    if (!wasmLoaded) return;

    try {
      setAst(parse(code));
    } catch (error) {
      setErrors(error as ValError[]);
    }
  }, [code, wasmLoaded]);

  useEffect(() => {
    if (editorRef.current?.view && !editorView) {
      setEditorView(editorRef.current.view);
    }
  }, [editorRef.current?.view, editorView]);

  // Save code to localStorage whenever it changes
  useEffect(() => {
    localStorage.setItem(STORAGE_KEY_CODE, code);
  }, [code]);

  // Save selected example to localStorage whenever it changes
  useEffect(() => {
    localStorage.setItem(STORAGE_KEY_EXAMPLE, currentExample);
  }, [currentExample]);

  const handleExampleChange = (value: string) => {
    setCurrentExample(value);
    setCode(EXAMPLES[value as keyof typeof EXAMPLES]);
  };

  if (!wasmLoaded) return null;

  return (
    <div className='flex h-screen flex-col p-4'>
      <div className='mb-4 flex items-center'>
        <a href='/' className='font-semibold'>
          val
        </a>
      </div>
      <ResizablePanelGroup
        direction='horizontal'
        className='min-h-0 flex-grow overflow-hidden rounded border'
      >
        <ResizablePanel
          defaultSize={50}
          minSize={30}
          className='flex min-h-0 flex-col overflow-hidden'
        >
          <div className='flex h-full flex-col overflow-hidden'>
            <div className='flex h-full min-h-0 flex-col overflow-hidden'>
              <div className='flex items-center justify-between border-b bg-gray-50 px-2 py-1'>
                <div className='flex items-center'>
                  <Select
                    value={currentExample}
                    onValueChange={handleExampleChange}
                  >
                    <SelectTrigger className='h-7 w-36 bg-white text-sm'>
                      <SelectValue placeholder='Select example' />
                    </SelectTrigger>
                    <SelectContent>
                      {Object.keys(EXAMPLES).map((key) => (
                        <SelectItem key={key} value={key}>
                          {key}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
                <EditorSettingsDialog />
              </div>
              <div className='h-full min-h-0 flex-grow overflow-hidden'>
                <Editor
                  errors={errors}
                  onChange={setCode}
                  onEditorReady={handleEditorReady}
                  ref={editorRef}
                  value={code}
                />
              </div>
            </div>
          </div>
        </ResizablePanel>
        <ResizableHandle withHandle />
        <ResizablePanel
          defaultSize={50}
          minSize={30}
          className='min-h-0 overflow-hidden'
        >
          <div className='h-full overflow-auto p-2'>
            {ast ? (
              <AstNode node={ast} editorView={editorView} />
            ) : (
              <div className='text-muted-foreground p-2'>No AST available</div>
            )}
          </div>
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
}

export default App;
