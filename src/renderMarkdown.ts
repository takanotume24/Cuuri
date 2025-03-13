import { Marked } from "marked";
import { Markdown, Html } from "./types";
import DOMPurify from "dompurify";

export function renderMarkdown(markdownText: Markdown): Html {
  const marked = new Marked();

  marked.use({
    async: false,
  });

  const html = marked.parse(markdownText) as string;
  const sanitized_html = DOMPurify.sanitize(html) as Html;

  return sanitized_html;
}
