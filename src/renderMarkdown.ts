import { marked } from "marked";

export async function renderMarkdown(markdownText: string): Promise<string> {
  return Promise.resolve(marked(markdownText));
}
