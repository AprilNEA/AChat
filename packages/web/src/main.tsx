import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { App } from '@achat/app';
import { createWebBridge } from './platform';

const bridge = createWebBridge();

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App bridge={bridge} />
  </StrictMode>,
);
