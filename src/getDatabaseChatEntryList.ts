import { invoke } from "@tauri-apps/api/core";
import { DatabaseChatEntry, RawDatabaseChatEntry } from "./types";
import { getDatabaseChatEntryFromRawDatabaseChatEntry } from "./getDatabaseChatEntryFromRawDatabaseChatEntry";

export async function getDatabaseChatEntryList(): Promise<
  DatabaseChatEntry[] | null
> {
  try {
    const rawDatabaseChatEntry: RawDatabaseChatEntry[] = await invoke(
      "get_chat_history"
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
