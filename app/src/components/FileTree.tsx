import { useMemo, useRef, useState } from "react";
import type { MouseEvent } from "react";
import type { TreeEntry } from "../api";
import { useI18n } from "../i18n";

interface Props {
  entries: TreeEntry[];
  selectedPaths: string[];
  activePath: string | null;
  onSelectionChange: (entries: TreeEntry[]) => void;
  onOpen: (entry: TreeEntry) => void;
}

function flattenAll(entries: TreeEntry[]): TreeEntry[] {
  const out: TreeEntry[] = [];
  const walk = (items: TreeEntry[]) => {
    for (const entry of items) {
      out.push(entry);
      if (entry.children?.length) walk(entry.children);
    }
  };
  walk(entries);
  return out;
}

function flattenVisible(entries: TreeEntry[], expanded: Set<string>): TreeEntry[] {
  const out: TreeEntry[] = [];
  const walk = (items: TreeEntry[]) => {
    for (const entry of items) {
      out.push(entry);
      if (entry.kind === "dir" && expanded.has(entry.path) && entry.children?.length) {
        walk(entry.children);
      }
    }
  };
  walk(entries);
  return out;
}

export function FileTree(props: Props) {
  const { t } = useI18n();
  const [expanded, setExpanded] = useState<Set<string>>(() => new Set());
  const anchorPath = useRef<string | null>(null);
  const allEntries = useMemo(() => flattenAll(props.entries), [props.entries]);
  const entriesByPath = useMemo(() => new Map(allEntries.map((entry) => [entry.path, entry])), [allEntries]);

  const toggle = (path: string) =>
    setExpanded((prev) => {
      const next = new Set(prev);
      if (next.has(path)) next.delete(path);
      else next.add(path);
      return next;
    });

  const select = (entry: TreeEntry, event: MouseEvent<HTMLButtonElement>) => {
    const toggleSelection = event.ctrlKey || event.metaKey;
    const selected = new Set(props.selectedPaths);

    if (event.shiftKey && anchorPath.current) {
      const visible = flattenVisible(props.entries, expanded);
      const start = visible.findIndex((item) => item.path === anchorPath.current);
      const end = visible.findIndex((item) => item.path === entry.path);
      if (start >= 0 && end >= 0) {
        const [from, to] = start <= end ? [start, end] : [end, start];
        const range = visible.slice(from, to + 1);
        const paths = toggleSelection ? new Set(props.selectedPaths) : new Set<string>();
        for (const item of range) paths.add(item.path);
        props.onSelectionChange([...paths].map((path) => entriesByPath.get(path)).filter(Boolean) as TreeEntry[]);
        return;
      }
    }

    anchorPath.current = entry.path;
    if (toggleSelection) {
      if (selected.has(entry.path)) selected.delete(entry.path);
      else selected.add(entry.path);
      props.onSelectionChange(
        [...selected].map((path) => entriesByPath.get(path)).filter(Boolean) as TreeEntry[],
      );
      return;
    }

    props.onSelectionChange([entry]);
    if (entry.kind === "dir") toggle(entry.path);
    else props.onOpen(entry);
  };

  if (props.entries.length === 0) {
    return <p className="tree-empty">{t.treeEmpty}</p>;
  }

  const selected = new Set(props.selectedPaths);
  return (
    <ul className="tree" role="tree" aria-multiselectable="true">
      {props.entries.map((entry) => (
        <Node
          key={entry.path}
          entry={entry}
          depth={0}
          expanded={expanded}
          toggle={toggle}
          selectedPaths={selected}
          activePath={props.activePath}
          onClick={select}
        />
      ))}
    </ul>
  );
}

interface NodeProps {
  entry: TreeEntry;
  depth: number;
  expanded: Set<string>;
  toggle: (path: string) => void;
  selectedPaths: Set<string>;
  activePath: string | null;
  onClick: (entry: TreeEntry, event: MouseEvent<HTMLButtonElement>) => void;
}

function Node(props: NodeProps) {
  const { entry, depth, expanded, selectedPaths, activePath, onClick } = props;
  const isDir = entry.kind === "dir";
  const isOpen = isDir && expanded.has(entry.path);
  const isSelected = selectedPaths.has(entry.path);
  const label = entry.kind === "note" ? entry.name.replace(/\.(md|markdown)$/i, "") : entry.name;
  const classes = [
    "tree-item",
    `kind-${entry.kind}`,
    isSelected ? "selected" : "",
    entry.path === activePath ? "active" : "",
  ].join(" ");

  return (
    <li
      role="treeitem"
      aria-expanded={isDir ? isOpen : undefined}
      aria-selected={isSelected}
    >
      <button
        type="button"
        className={classes}
        style={{ paddingLeft: `${0.5 + depth * 0.9}rem` }}
        title={entry.path}
        onClick={(event) => onClick(entry, event)}
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
