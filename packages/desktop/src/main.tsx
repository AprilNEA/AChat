import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { App } from '@achat/app';
import { createDesktopBridge } from './platform';

const bridge = createDesktopBridge();

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App bridge={bridge} />
  </StrictMode>,
);
