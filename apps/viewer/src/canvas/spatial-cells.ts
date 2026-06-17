export type Rect = { x: number; y: number; width: number; height: number };

export function insert<T>(cells: Map<string, Set<T>>, rect: Rect, item: T, size: number): void {
  forEachCell(rect, size, (key) => {
    const bucket = cells.get(key) ?? new Set<T>();
    bucket.add(item);
    cells.set(key, bucket);
  });
}

export function collect<T>(cells: Map<string, Set<T>>, rect: Rect, size: number): Set<T> {
  const result = new Set<T>();
  forEachCell(rect, size, (key) => cells.get(key)?.forEach((item) => result.add(item)));
  return result;
}

function forEachCell(rect: Rect, size: number, fn: (key: string) => void): void {
  const range = cellRange(rect, size);
  for (let y = range.minY; y <= range.maxY; y += 1) {
    for (let x = range.minX; x <= range.maxX; x += 1) {
      fn(`${x}:${y}`);
    }
  }
}

function cellRange(rect: Rect, size: number) {
  return {
    minX: Math.floor(rect.x / size),
    maxX: Math.floor((rect.x + rect.width) / size),
    minY: Math.floor(rect.y / size),
    maxY: Math.floor((rect.y + rect.height) / size)
  };
}
