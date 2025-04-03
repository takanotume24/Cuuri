import { RawDatabaseChatEntry, DatabaseChatEntry } from "./types.ts";
import dayjs from "npm:dayjs";
import { renderMarkdown } from "./renderMarkdown.ts";

export function getDatabaseChatEntryFromRawDatabaseChatEntry(
  rawDatabaseChatEntry: RawDatabaseChatEntry
) {
  const databaseChatEntry: DatabaseChatEntry = {
    session_id: rawDatabaseChatEntry.session_id,
    question: rawDatabaseChatEntry.question,
    answer: renderMarkdown(rawDatabaseChatEntry.answer),
    created_at: dayjs(rawDatabaseChatEntry.created_at),
  };
  return databaseChatEntry;
}
