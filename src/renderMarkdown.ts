import { Marked } from "marked";
import markedKatex from "marked-katex-extension";
import "katex/dist/katex.min.css";
import { Markdown } from "./types";

export function renderMarkdown(markdownText: Markdown): string {
  const marked = new Marked();

  marked.use({
    async: false,
  });

  marked.use(
    markedKatex({
      throwOnError: false,
    })
  );

  const html = marked.parse(markdownText) as string;

  return html;
}
