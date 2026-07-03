import { useCallback, useEffect, useRef } from "react";
import type { CSSProperties } from "react";
import CodeMirror, { type ReactCodeMirrorRef } from "@uiw/react-codemirror";
import { EditorView, keymap } from "@codemirror/view";
import { indentUnit } from "@codemirror/language";
import { indentWithTab } from "@codemirror/commands";
import { useEditorStore } from "@/stores/editorStore";
import { useSettingsStore } from "@/stores/settingsStore";
import { cmLanguage } from "@/utils/cmLanguage";
import { editorExtensions } from "@/styles/cmTheme";
import Welcome from "@/components/Welcome/Welcome";

export default function Editor() {
  const activeTab = useEditorStore((s) => s.activeTab());
  const updateContent = useEditorStore((s) => s.updateContent);
  const setCursor = useEditorStore((s) => s.setCursor);
  const saveTab = useEditorStore((s) => s.saveTab);
  const pendingJumpLine = useEditorStore((s) => s.pendingJumpLine);
  const clearJump = useEditorStore((s) => s.clearJump);
  const config = useSettingsStore((s) => s.config);
  const ref = useRef<ReactCodeMirrorRef>(null);

  // Ctrl+S zapisuje aktywna karte (dziala nawet gdy focus jest w edytorze,
  // bo react-codemirror przepuszcza zdarzenia klawiatury nieobsluzone przez CM).
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "s" && activeTab) {
        e.preventDefault();
        if (!activeTab.path) return; // App.tsx obsluguje "zapisz jako" dla nowych plikow
        saveTab(activeTab.id);
      }
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [activeTab, saveTab]);

  // Auto-save z prostym debounce, gdy wlaczone w ustawieniach.
  useEffect(() => {
    if (!config.auto_save || !activeTab || !activeTab.isDirty || !activeTab.path) return;
    const t = setTimeout(() => saveTab(activeTab.id), 900);
    return () => clearTimeout(t);
  }, [config.auto_save, activeTab, saveTab]);

  useEffect(() => {
    if (pendingJumpLine == null || !ref.current?.view) return;
    const view = ref.current.view;
    const line = Math.min(pendingJumpLine, view.state.doc.lines);
    const pos = view.state.doc.line(line).from;
    view.dispatch({
      selection: { anchor: pos },
      effects: EditorView.scrollIntoView(pos, { y: "center" }),
    });
    view.focus();
    clearJump();
  }, [pendingJumpLine, clearJump]);

  const onChange = useCallback(
    (value: string) => {
      if (activeTab) updateContent(activeTab.id, value);
    },
    [activeTab, updateContent],
  );

  const onUpdate = useCallback(
    (viewUpdate: { view: EditorView }) => {
      if (!activeTab) return;
      const sel = viewUpdate.view.state.selection.main;
      const line = viewUpdate.view.state.doc.lineAt(sel.head);
      setCursor(activeTab.id, line.number, sel.head - line.from + 1);
    },
    [activeTab, setCursor],
  );

  if (!activeTab) {
    return <Welcome />;
  }

  const wrapperStyle = {
    height: "100%",
    overflow: "hidden",
    "--cm-font-size": `${config.font_size}px`,
  } as CSSProperties;

  return (
    <div style={wrapperStyle}>
      <CodeMirror
        ref={ref}
        key={activeTab.id}
        value={activeTab.content}
        height="100%"
        theme="none"
        basicSetup={{
          lineNumbers: config.show_line_numbers,
          foldGutter: true,
          highlightActiveLine: true,
          autocompletion: config.autocomplete_enabled,
          closeBrackets: true,
          bracketMatching: true,
          indentOnInput: true,
        }}
        extensions={[
          ...editorExtensions,
          ...cmLanguage(activeTab.language),
          indentUnit.of(" ".repeat(config.tab_size)),
          keymap.of([indentWithTab]),
          config.word_wrap ? EditorView.lineWrapping : [],
        ]}
        onChange={onChange}
        onUpdate={onUpdate}
      />
    </div>
  );
}
