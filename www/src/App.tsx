import { Suspense, lazy } from 'react';
import { Navigate, Route, Routes, useLocation } from 'react-router';

import Home from './pages/home';

const Documentation = lazy(() => import('./pages/documentation'));
const Playground = lazy(() => import('./pages/playground'));

function App() {
  const { hash } = useLocation();

  return (
    <Suspense
      fallback={
        <div
          role='status'
          className='text-muted-foreground flex h-dvh items-center justify-center'
        >
          Loading…
        </div>
      }
    >
      <Routes>
        <Route path='/' element={<Home />} />
        <Route path='/documentation' element={<Documentation />} />
        <Route
          path='/docs'
          element={<Navigate to={`/documentation${hash}`} replace />}
        />
        <Route path='/playground' element={<Playground />} />
        <Route path='*' element={<Navigate to='/' replace />} />
      </Routes>
    </Suspense>
  );
}

export default App;
