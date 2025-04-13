import { useEffect, useState } from 'react';
import { toast } from 'sonner';
import init, { add } from 'val-wasm';

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

  return <h1 className='text-3xl font-bold underline'>{add(1, 2)}</h1>;
}

export default App;
