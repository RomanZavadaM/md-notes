import { useState } from "react";
import type { TreeEntry } from "../api";
import { useI18n } from "../i18n";

interface Props {
  entries: TreeEntry[];
  selectedPath: string | null;
  activePath: string | null;
  onSelect: (entry: TreeEntry) => void;
  onOpen: (entry: TreeEntry) => void;
}

export function FileTree(props: Props) {
  const { t } = useI18n();
  const [expanded, setExpanded] = useState<Set<string>>(() => new Set());

  const toggle = (path: string) =>
    setExpanded((prev) => {
      const next = new Set(prev);
      if (next.has(path)) next.delete(path);
      else next.add(path);
      return next;
    });

  if (props.entries.length === 0) {
    return <p className="tree-empty">{t.treeEmpty}</p>;
  }
  return (
    <ul className="tree" role="tree">
      {props.entries.map((entry) => (
        <Node key={entry.path} entry={entry} depth={0} expanded={expanded} toggle={toggle} {...props} />
      ))}
    </ul>
  );
}

interface NodeProps extends Props {
  entry: TreeEntry;
  depth: number;
  expanded: Set<string>;
  toggle: (path: string) => void;
}

function Node(props: NodeProps) {
  const { entry, depth, expanded, toggle, selectedPath, activePath, onSelect, onOpen } = props;
  const isDir = entry.kind === "dir";
  const isOpen = isDir && expanded.has(entry.path);
  const label = entry.kind === "note" ? entry.name.replace(/\.(md|markdown)$/i, "") : entry.name;
  const classes = [
    "tree-item",
    `kind-${entry.kind}`,
    entry.path === selectedPath ? "selected" : "",
    entry.path === activePath ? "active" : "",
  ].join(" ");

  return (
    <li role="treeitem" aria-expanded={isDir ? isOpen : undefined}>
      <button
        type="button"
        className={classes}
        style={{ paddingLeft: `${0.5 + depth * 0.9}rem` }}
        title={entry.path}
        onClick={() => {
          onSelect(entry);
          if (isDir) toggle(entry.path);
          else onOpen(entry);
        }}
      >
        <span className="tree-icon" aria-hidden>
          {isDir ? (isOpen ? "▾" : "▸") : entry.kind === "note" ? "•" : "◦"}
        </span>
        <span className="tree-label">{label}</span>
      </button>
      {isOpen && entry.children && entry.children.length > 0 && (
        <ul role="group">
          {entry.children.map((child) => (
            <Node key={child.path} {...props} entry={child} depth={depth + 1} />
          ))}
        </ul>
      )}
    </li>
  );
}
