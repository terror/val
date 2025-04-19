import { AstNode } from '@/components/ast-node';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { useEffect, useState } from 'react';
import { toast } from 'sonner';
import init, { evaluate, parse } from 'val-wasm';
import type { AstNode as AstNodeType, ValError } from '@/lib/types'

import { Editor } from './components/editor';
import { EditorSettingsDialog } from './components/editor-settings-dialog';
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from './components/ui/resizable';
import EXAMPLES from './lib/examples';

function App() {
  const [ast, setAst] = useState<AstNodeType | null>(null);
  const [code, setCode] = useState(EXAMPLES.factorial);
  const [currentExample, setCurrentExample] = useState('factorial');
  const [errors, setErrors] = useState<ValError[]>([]);
  const [wasmLoaded, setWasmLoaded] = useState(false);

  useEffect(() => {
    init()
      .then(() => {
        setWasmLoaded(true);
        setAst(parse(code));
      })
      .catch((error) => {
        toast.error(error);
      });
  }, []);

  useEffect(() => {
    if (!wasmLoaded) return;

    try {
      setAst(parse(code));
      console.log(evaluate(code));
    } catch (error) {
      setErrors(error as ValError[]);
    }
  }, [code]);

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
                <Editor errors={errors} onChange={setCode} value={code} />
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
              <AstNode node={ast} />
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
