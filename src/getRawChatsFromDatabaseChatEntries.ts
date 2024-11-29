import { RawChats, DatabaseChatEntry, RawChatEntry } from "./types";

export function getRawChatsFromDatabaseChatEntries(
  databaseChatEntries: DatabaseChatEntry[]
): RawChats {
  let rawChats: RawChats = {};
  for (const databaseChatEntry of databaseChatEntries) {
    if (!rawChats[databaseChatEntry.session_id]) {
      rawChats[databaseChatEntry.session_id] = [];
    }
    const rawChatEntry: RawChatEntry = {
      question: databaseChatEntry.question,
      answer: databaseChatEntry.answer,
    };
    rawChats[databaseChatEntry.session_id].push(rawChatEntry);
  }

  return rawChats;
}
