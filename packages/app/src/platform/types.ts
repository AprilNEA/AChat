/**
 * Platform Bridge — the abstraction layer between the shared React app
 * and platform-specific backends (Tauri IPC vs HTTP to cloud).
 */
export interface PlatformBridge {
  readonly platform: 'web' | 'desktop';
  readonly capabilities: PlatformCapabilities;

  // Chat
  sendMessage(conversationId: string, content: string, modelId: string): Promise<string>;
  getConversations(limit: number, offset: number): Promise<Conversation[]>;
  getMessages(conversationId: string): Promise<Message[]>;
  streamChat(conversationId: string, prompt: string, modelId: string): AsyncIterable<AgentEvent>;

  // Agent
  runAgent(sessionId: string, prompt: string): AsyncIterable<AgentEvent>;
  stopAgent(sessionId: string): Promise<void>;

  // Settings
  getSettings(): Promise<Settings>;
  updateSettings(settings: Partial<Settings>): Promise<void>;

  // Providers
  listProviders(): Promise<Provider[]>;
  addProvider(config: ProviderConfig): Promise<Provider>;

  // Sync (desktop only — no-op on web)
  triggerSync(): Promise<void>;
  getSyncStatus(): Promise<SyncStatus>;
}

export interface PlatformCapabilities {
  /** Agent can run locally in the app process (desktop only) */
  localAgent: boolean;
  /** Tools can access the local filesystem (desktop only) */
  fileSystemTools: boolean;
  /** App works without internet (desktop only) */
  offlineMode: boolean;
  /** Can offload agent work to cloud compute */
  cloudCompute: boolean;
}

// --- Shared types (mirrors Rust achat-core types) ---

export interface Conversation {
  id: string;
  title: string;
  modelId?: string;
  teamId?: string;
  isShared: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface Message {
  id: string;
  conversationId: string;
  parentId?: string;
  role: 'user' | 'assistant' | 'system' | 'tool';
  content: MessageContent;
  metadata?: Record<string, unknown>;
  createdAt: string;
}

export type MessageContent =
  | { type: 'text'; text: string }
  | { type: 'tool_call'; id: string; name: string; arguments: unknown }
  | { type: 'tool_result'; id: string; content: string }
  | { type: 'image'; url: string; alt?: string };

export type AgentEvent =
  | { type: 'thinking' }
  | { type: 'stream_chunk'; content: string }
  | { type: 'tool_call_start'; name: string; arguments: unknown }
  | { type: 'tool_call_end'; name: string; result: string }
  | { type: 'completed'; messageId: string }
  | { type: 'error'; message: string };

export interface Provider {
  id: string;
  name: string;
  providerType: 'openai' | 'anthropic' | 'ollama' | 'custom';
  isEnabled: boolean;
}

export interface ProviderConfig {
  name: string;
  providerType: string;
  baseUrl?: string;
  apiKey?: string;
}

export interface Settings {
  theme: 'light' | 'dark' | 'system';
  defaultModel?: string;
  syncEnabled: boolean;
  cloudUrl?: string;
}

export interface SyncStatus {
  connected: boolean;
  lastSyncAt?: string;
  pendingChanges: number;
}
