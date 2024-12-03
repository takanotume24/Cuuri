type Id<T extends string> = string & { readonly brand: T };
export type SessionId = Id<"SessionId">;

type ReadOnlyBrand<T, B> = T & { readonly __brand: B };
export type Markdown = ReadOnlyBrand<string, "Markdown">;
export type Html = ReadOnlyBrand<string, "Html">;
export type UserInput = ReadOnlyBrand<string, "UserInput">;

export type ModelName = ReadOnlyBrand<string, "ModelName">;
export type ApiKey = ReadOnlyBrand<string, "ApiKey">;
export type EncodedImage = ReadOnlyBrand<string, "EncodedImage">;

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
