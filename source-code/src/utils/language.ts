export const LANGUAGE_BY_EXTENSION: Record<string, string> = {
  hl: "Hacker Lang",
  hlpp: "Hacker Lang++",
  "hl++": "Hacker Lang++",
  hs: "H#",
  c: "C",
  h: "C",
  cpp: "C++",
  cc: "C++",
  cxx: "C++",
  hpp: "C++",
  py: "Python",
  go: "Go",
  odin: "Odin",
  cr: "Crystal",
  sh: "Shell",
  bash: "Shell",
  zsh: "Shell",
  vala: "Vala",
  nim: "Nim",
  html: "HTML",
  htm: "HTML",
  css: "CSS",
  js: "JavaScript",
  jsx: "JavaScript",
  ts: "TypeScript",
  tsx: "TypeScript",
  dart: "Dart",
  kt: "Kotlin",
  kts: "Kotlin",
  lua: "Lua",
  rs: "Rust",
  java: "Java",
  yaml: "YAML",
  yml: "YAML",
  json: "JSON",
  toml: "TOML",
  hcl: "HCL",
  tf: "HCL",
  xml: "XML",
  hk: "HK Plugin",
  md: "Markdown",
  markdown: "Markdown",
};

export function languageFromPath(path: string): string {
  const name = path.split(/[/\\]/).pop() ?? path;
  const dot = name.lastIndexOf(".");
  if (dot === -1) return "Plain Text";
  const ext = name.slice(dot + 1).toLowerCase();
  return LANGUAGE_BY_EXTENSION[ext] ?? "Plain Text";
}

export function fileIcon(path: string, isDir: boolean): string {
  if (isDir) return "📁";
  const lang = languageFromPath(path);
  const icons: Record<string, string> = {
    Rust: "🦀",
    Python: "🐍",
    JavaScript: "📜",
    TypeScript: "📘",
    HTML: "🌐",
    CSS: "🎨",
    JSON: "🧩",
    YAML: "⚙️",
    TOML: "⚙️",
    Markdown: "📝",
    Shell: "💲",
    "HK Plugin": "🧬",
    "Hacker Lang": "🖥️",
    "Hacker Lang++": "🖥️",
    "H#": "🖥️",
  };
  return icons[lang] ?? "📄";
}
