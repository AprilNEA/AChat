import { createBrowserRouter, createMemoryRouter } from 'react-router';
import type { PlatformBridge } from './platform';
import { RootLayout } from './routes/root-layout';
import { ChatLayout } from './routes/chat/layout';
import { ChatPage } from './routes/chat/page';
import { ConversationPage } from './routes/chat/conversation';
import { SettingsPage } from './routes/settings/page';

const routes = [
  {
    path: '/',
    element: <RootLayout />,
    children: [
      {
        index: true,
        element: <ChatPage />,
      },
      {
        path: 'chat',
        element: <ChatLayout />,
        children: [
          {
            index: true,
            element: <ChatPage />,
          },
          {
            path: ':conversationId',
            element: <ConversationPage />,
          },
        ],
      },
      {
        path: 'settings',
        element: <SettingsPage />,
      },
    ],
  },
];

export function createAppRouter(bridge: PlatformBridge) {
  // Desktop uses memory router (no URL bar), web uses browser router
  if (bridge.platform === 'desktop') {
    return createMemoryRouter(routes);
  }
  return createBrowserRouter(routes);
}
