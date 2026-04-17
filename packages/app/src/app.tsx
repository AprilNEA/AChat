import { createContext, useContext } from 'react';
import { RouterProvider } from 'react-router';
import type { PlatformBridge } from './platform';
import { createAppRouter } from './router';

const PlatformContext = createContext<PlatformBridge | null>(null);

export function usePlatform(): PlatformBridge {
  const bridge = useContext(PlatformContext);
  if (!bridge) {
    throw new Error('usePlatform must be used within a PlatformProvider');
  }
  return bridge;
}

export function App({ bridge }: { bridge: PlatformBridge }) {
  const router = createAppRouter(bridge);

  return (
    <PlatformContext.Provider value={bridge}>
      <RouterProvider router={router} />
    </PlatformContext.Provider>
  );
}
