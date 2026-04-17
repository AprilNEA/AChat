import { useParams } from 'react-router';

export function ConversationPage() {
  const { conversationId } = useParams<{ conversationId: string }>();

  return (
    <div className="flex-1 flex flex-col">
      <header className="border-b px-4 py-3">
        <h2 className="font-semibold">Conversation</h2>
        <p className="text-xs text-muted-foreground">{conversationId}</p>
      </header>
      <div className="flex-1 overflow-y-auto p-4">
        {/* TODO: Message list */}
      </div>
      <div className="border-t p-4">
        {/* TODO: Chat input */}
      </div>
    </div>
  );
}
