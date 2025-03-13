import { Marked } from "marked";
import { Markdown } from "./types";
import DOMPurify from "dompurify";

export function renderMarkdown(markdownText: Markdown): string {
  const marked = new Marked();

  marked.use({
    async: false,
  });

  const html = marked.parse(markdownText) as string;
  const sanitized_html = DOMPurify.sanitize(html);

  return sanitized_html;
}
