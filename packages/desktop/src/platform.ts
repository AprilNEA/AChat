import { invoke } from '@tauri-apps/api/core';
import type { PlatformBridge } from '@achat/app/platform';

/**
 * Desktop platform bridge — all calls go to Tauri Rust backend via IPC invoke.
 */
export function createDesktopBridge(): PlatformBridge {
  return {
    platform: 'desktop',
    capabilities: {
      localAgent: true,
      fileSystemTools: true,
      offlineMode: true,
      cloudCompute: false, // Set to true after cloud config
    },

    // Chat
    async sendMessage(conversationId, content, modelId) {
      return invoke<string>('send_message', { conversationId, content, modelId });
    },

    async getConversations(limit, offset) {
      return invoke('get_conversations', { limit, offset });
    },

    async getMessages(conversationId) {
      return invoke('get_messages', { conversationId });
    },

    async *streamChat(_conversationId, _prompt, _modelId) {
      // TODO: invoke run_agent + listen to Tauri events
    },

    // Agent
    async *runAgent(_sessionId, _prompt) {
      // TODO: invoke run_agent + listen to Tauri events via @tauri-apps/api/event
    },

    async stopAgent(sessionId) {
      return invoke('stop_agent', { sessionId });
    },

    // Settings
    async getSettings() {
      return invoke('get_settings');
    },

    async updateSettings(settings) {
      return invoke('update_settings', { settings });
    },

    // Providers
    async listProviders() {
      return invoke('list_providers');
    },

    async addProvider(config) {
      return invoke('add_provider', { config });
    },

    // Sync
    async triggerSync() {
      return invoke('trigger_sync');
    },

    async getSyncStatus() {
      return invoke('get_sync_status');
    },
  };
}
