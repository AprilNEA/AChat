import { Outlet } from 'react-router';

export function RootLayout() {
  return (
    <div className="flex h-screen w-screen">
      <Outlet />
    </div>
  );
}
