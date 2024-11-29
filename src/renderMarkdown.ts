import { marked } from "marked";
import { Markdown, Html } from "./types";

export function renderMarkdown(markdownText: Markdown): Html {
  const html = marked(markdownText) as Html;
  return html;
}
