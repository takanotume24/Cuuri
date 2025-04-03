import { invoke } from "npm:@tauri-apps/api/core";
import { RawDatabaseChatEntry } from "./types.ts";
import { DatabaseChatEntry } from "./types.ts";
import { getDatabaseChatEntryFromRawDatabaseChatEntry } from "./getDatabaseChatEntryFromRawDatabaseChatEntry.ts";
import { SessionId } from "./types.ts";

export async function getDatabaseChatEntryBySession(session_id: SessionId) {
  try {
    const rawDatabaseChatEntry: RawDatabaseChatEntry[] = await invoke(
      "get_chat_history_by_session",
      {
        targetSessionId: session_id,
      }
    );

    const databaseChatEntryList: DatabaseChatEntry[] = rawDatabaseChatEntry.map(
      getDatabaseChatEntryFromRawDatabaseChatEntry
    );
    return databaseChatEntryList;
  } catch (error) {
    console.error("Failed to get chat history:", error);
    return null;
  }
}
