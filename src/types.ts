type Id<T extends string> = string & { readonly brand: T };
export type SessionId = Id<"SessionId">;

type Brand<T, B> = T & { readonly __brand: B };
export type Markdown = Brand<string, "Markdown">;
export type Html = Brand<string, "Html">;
export type UserInput = Brand<string, "UserInput">;

export type ModelName = Brand<string, "ModelName">;
export type ApiKey = Brand<string, "ApiKey">;

export interface DatabaseChatEntry {
  session_id: SessionId;
  question: UserInput;
  answer: Markdown;
  created_at: string;
}

export interface RawChatEntry {
  question: UserInput;
  answer: Markdown;
}

export interface HtmlChatEntry {
  question: UserInput;
  answer: Html;
}

export interface RawChats {
  [sessionId: SessionId]: Array<RawChatEntry>;
}
