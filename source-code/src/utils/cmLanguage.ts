import type { Extension } from "@codemirror/state";
import { javascript } from "@codemirror/lang-javascript";
import { python } from "@codemirror/lang-python";
import { rust } from "@codemirror/lang-rust";
import { css } from "@codemirror/lang-css";
import { html } from "@codemirror/lang-html";
import { json } from "@codemirror/lang-json";
import { xml } from "@codemirror/lang-xml";
import { yaml } from "@codemirror/lang-yaml";
import { markdown } from "@codemirror/lang-markdown";
import { java } from "@codemirror/lang-java";
import { cpp } from "@codemirror/lang-cpp";

/**
 * CodeMirror ma gotowe gramatyki tylko dla popularnych jezykow. Dla jezykow
 * wlasnych HackerOS (Hacker Lang, H# ...) i mniej popularnych (Nim, Crystal,
 * Odin, Vala, Dart, Kotlin, Lua, HCL) uzywamy pustego rozszerzenia — plik
 * nadal edytuje sie normalnie, tylko bez kolorowania skladni. To jeden z
 * punktow rozbudowy wypisanych w README (wlasne gramatyki @lezer).
 */
export function cmLanguage(language: string): Extension[] {
  switch (language) {
    case "JavaScript":
    case "TypeScript":
      return [javascript({ jsx: true, typescript: language === "TypeScript" })];
    case "Python":
      return [python()];
    case "Rust":
      return [rust()];
    case "CSS":
      return [css()];
    case "HTML":
      return [html()];
    case "JSON":
      return [json()];
    case "XML":
    case "HCL":
      return [xml()];
    case "YAML":
      return [yaml()];
    case "Markdown":
      return [markdown()];
    case "Java":
    case "Kotlin":
      return [java()];
    case "C":
    case "C++":
      return [cpp()];
    default:
      return [];
  }
}
