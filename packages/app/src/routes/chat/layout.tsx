import { Outlet } from 'react-router';

export function ChatLayout() {
  return (
    <>
      {/* TODO: Sidebar with conversation list */}
      <aside className="w-64 border-r bg-muted/50">
        <div className="p-4 font-semibold">Conversations</div>
      </aside>
      <main className="flex-1 flex flex-col">
        <Outlet />
      </main>
    </>
  );
}
