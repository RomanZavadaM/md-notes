import type { RelationTreeNode } from "../api";

export const NODE_W = 214;
export const NODE_H = 48;
export const COL_GAP = 52;
export const ROW_GAP = 12;

export interface PlacedNode {
  node: RelationTreeNode;
  /** Position in the tree (the chain of paths), unique even for repeated notes. */
  key: string;
  parentKey: string | null;
  depth: number;
  x: number;
  y: number;
  /** The branch is folded: its children are not drawn. */
  folded: boolean;
  /** The node has children that can be folded. */
  foldable: boolean;
}

export interface TreeLayout {
  placed: PlacedNode[];
  width: number;
  height: number;
}

/**
 * Lays out a relation tree in columns: one column per depth, leaves on their own
 * rows, a parent centred against its children. With `mirror` the root is on the
 * right and the chain grows to the left, so ownership reads left to right
 * towards the subject.
 */
export function layoutTree(root: RelationTreeNode, folded: Set<string>, mirror: boolean): TreeLayout {
  const items: PlacedNode[] = [];
  const rows: number[] = [];
  let nextRow = 0;
  let deepest = 0;

  const visit = (node: RelationTreeNode, depth: number, parentKey: string | null): number => {
    const key = parentKey === null ? node.path : `${parentKey}>${node.path}`;
    const foldable = node.children.length > 0;
    const isFolded = foldable && folded.has(key);
    const children = isFolded ? [] : node.children;
    const index = items.length;
    deepest = Math.max(deepest, depth);
    items.push({ node, key, parentKey, depth, x: 0, y: 0, folded: isFolded, foldable });
    rows.push(0);
    let row: number;
    if (children.length === 0) {
      row = nextRow;
      nextRow += 1;
    } else {
      const childRows = children.map((child) => visit(child, depth + 1, key));
      row = (childRows[0] + childRows[childRows.length - 1]) / 2;
    }
    rows[index] = row;
    return row;
  };
  visit(root, 0, null);

  items.forEach((item, index) => {
    const column = mirror ? deepest - item.depth : item.depth;
    item.x = column * (NODE_W + COL_GAP);
    item.y = rows[index] * (NODE_H + ROW_GAP);
  });
  const width = (deepest + 1) * NODE_W + deepest * COL_GAP;
  const height = Math.max(1, nextRow) * (NODE_H + ROW_GAP) - ROW_GAP;
  return { placed: items, width, height };
}

/** Keys of every node with children, for "fold all". */
export function foldableKeys(root: RelationTreeNode): string[] {
  const keys: string[] = [];
  const visit = (node: RelationTreeNode, parentKey: string | null) => {
    const key = parentKey === null ? node.path : `${parentKey}>${node.path}`;
    if (node.children.length > 0) keys.push(key);
    node.children.forEach((child) => visit(child, key));
  };
  visit(root, null);
  return keys;
}

/** Cubic line from the parent box to the child box, towards the side the child lies on. */
export function edgePath(parent: PlacedNode, child: PlacedNode): string {
  const toRight = child.x > parent.x;
  const x1 = toRight ? parent.x + NODE_W : parent.x;
  const x2 = toRight ? child.x : child.x + NODE_W;
  const y1 = parent.y + NODE_H / 2;
  const y2 = child.y + NODE_H / 2;
  const mid = (x1 + x2) / 2;
  return `M${x1},${y1} C${mid},${y1} ${mid},${y2} ${x2},${y2}`;
}

/** Shortens long titles for the box; the full title stays in the tooltip. */
export function shortTitle(text: string, limit = 26): string {
  const chars = [...text];
  return chars.length <= limit ? text : `${chars.slice(0, limit - 1).join("")}…`;
}
