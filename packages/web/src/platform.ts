import type { PlatformBridge } from '@achat/app/platform';

/**
 * Web platform bridge — all calls go to the Axum cloud server via HTTP/SSE.
 */
export function createWebBridge(): PlatformBridge {
  const baseUrl = '/api/v1';

  return {
    platform: 'web',
    capabilities: {
      localAgent: false,
      fileSystemTools: false,
      offlineMode: false,
      cloudCompute: true,
    },

    // Chat
    async sendMessage(conversationId, content, modelId) {
      const res = await fetch(`${baseUrl}/chat/${conversationId}/messages`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ content, modelId }),
      });
      const data = await res.json();
      return data.id;
    },

    async getConversations(limit, offset) {
      const res = await fetch(`${baseUrl}/chat?limit=${limit}&offset=${offset}`);
      return res.json();
    },

    async getMessages(conversationId) {
      const res = await fetch(`${baseUrl}/chat/${conversationId}/messages`);
      return res.json();
    },

    async *streamChat(_conversationId, _prompt, _modelId) {
      // TODO: SSE streaming via EventSource or fetch + ReadableStream
    },

    // Agent
    async *runAgent(_sessionId, _prompt) {
      // TODO: POST to agent endpoint, consume SSE stream
    },

    async stopAgent(_sessionId) {
      // TODO: POST to agent stop endpoint
    },

    // Settings
    async getSettings() {
      return { theme: 'system', syncEnabled: false };
    },

    async updateSettings(_settings) {
      // TODO
    },

    // Providers
    async listProviders() {
      const res = await fetch(`${baseUrl}/providers`);
      return res.json();
    },

    async addProvider(config) {
      const res = await fetch(`${baseUrl}/providers`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(config),
      });
      return res.json();
    },

    // Sync — no-op on web
    async triggerSync() {},
    async getSyncStatus() {
      return { connected: true, pendingChanges: 0 };
    },
  };
}
