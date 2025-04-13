import { TreeViewer } from '@/components/tree-viewer';
import { useEffect, useState } from 'react';
import { toast } from 'sonner';
import init, { parse } from 'val-wasm';

function App() {
  const [wasmLoaded, setWasmLoaded] = useState(false);

  useEffect(() => {
    init()
      .then(() => {
        setWasmLoaded(true);
      })
      .catch((error) => {
        toast.error(error);
      });
  });

  if (!wasmLoaded) return null;

  return <TreeViewer ast={parse('1 + 1')} />;
}

export default App;
