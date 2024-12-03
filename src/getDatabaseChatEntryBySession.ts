import { invoke } from "@tauri-apps/api/core";
import { RawDatabaseChatEntry } from "./types";
import { DatabaseChatEntry } from "./types";
import { getDatabaseChatEntryFromRawDatabaseChatEntry } from "./getDatabaseChatEntryFromRawDatabaseChatEntry";
import { SessionId } from "./types";

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
