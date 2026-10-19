// Force-directed layout (Fruchterman-Reingold with cooling), no dependencies.
// Runs a few iterations per animation frame so the graph visibly "settles"
// like in Obsidian, and stays responsive for large vaults by lowering the
// number of iterations as the node count grows.

export type Edge = readonly [number, number];

export class ForceLayout {
  readonly x: Float64Array;
  readonly y: Float64Array;
  /** Iterations to run in total. */
  readonly total: number;
  /** Iterations per animation frame. */
  readonly perFrame: number;
  private readonly dx: Float64Array;
  private readonly dy: Float64Array;
  private iteration = 0;
  private readonly startTemperature: number;

  constructor(
    private readonly count: number,
    private readonly edges: readonly Edge[],
  ) {
    this.x = new Float64Array(count);
    this.y = new Float64Array(count);
    this.dx = new Float64Array(count);
    this.dy = new Float64Array(count);
    // Golden-angle spiral: a deterministic, evenly spread start.
    for (let i = 0; i < count; i += 1) {
      const radius = Math.sqrt(i + 0.5);
      const angle = i * 2.399963229728653;
      this.x[i] = Math.cos(angle) * radius;
      this.y[i] = Math.sin(angle) * radius;
    }
    const pairs = Math.max((count * count) / 2, 1);
    this.total = Math.max(60, Math.min(400, Math.floor(2e8 / pairs)));
    this.perFrame = Math.max(1, Math.min(10, Math.floor(3e6 / pairs)));
    this.startTemperature = 0.15 * Math.sqrt(count) + 0.6;
  }

  get done(): boolean {
    return this.iteration >= this.total || this.count < 2;
  }

  /** Runs one iteration. */
  step(): void {
    const n = this.count;
    const { x, y, dx, dy } = this;
    dx.fill(0);
    dy.fill(0);

    // Repulsion between all pairs: |F| = 1 / d.
    for (let i = 0; i < n; i += 1) {
      for (let j = i + 1; j < n; j += 1) {
        let ex = x[i] - x[j];
        let ey = y[i] - y[j];
        let d2 = ex * ex + ey * ey;
        if (d2 < 1e-4) {
          ex = (i - j) * 1e-2;
          ey = 1e-2;
          d2 = ex * ex + ey * ey;
        }
        const f = 1 / d2;
        dx[i] += ex * f;
        dy[i] += ey * f;
        dx[j] -= ex * f;
        dy[j] -= ey * f;
      }
    }

    // Attraction along edges: |F| = d^2.
    for (const [a, b] of this.edges) {
      const ex = x[a] - x[b];
      const ey = y[a] - y[b];
      const d = Math.sqrt(ex * ex + ey * ey) || 1e-3;
      const f = d;
      dx[a] -= ex * f;
      dy[a] -= ey * f;
      dx[b] += ex * f;
      dy[b] += ey * f;
    }

    // Weak gravity keeps disconnected parts in view.
    for (let i = 0; i < n; i += 1) {
      dx[i] -= x[i] * 0.02;
      dy[i] -= y[i] * 0.02;
    }

    const temperature = this.startTemperature * Math.max(0.02, 1 - this.iteration / this.total);
    for (let i = 0; i < n; i += 1) {
      const length = Math.sqrt(dx[i] * dx[i] + dy[i] * dy[i]);
      if (length > 0) {
        const move = Math.min(length, temperature) / length;
        x[i] += dx[i] * move;
        y[i] += dy[i] * move;
      }
    }
    this.iteration += 1;
  }
}
