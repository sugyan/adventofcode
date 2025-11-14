import { useEffect, useRef, useState, useMemo } from "react";

const GRID_SIZE = 71; // 0-70 inclusive
const CELL_SIZE = 8;

interface Position {
  x: number;
  y: number;
}

function parseInput(input: string): Position[] {
  return input
    .trim()
    .split("\n")
    .filter((line) => line.trim().length > 0)
    .map((line) => {
      const parts = line.trim().split(",");
      if (parts.length !== 2) {
        // Try to extract x,y from format like "   1→17,8"
        const match = line.match(/(\d+),(\d+)/);
        if (!match) throw new Error(`Invalid line: ${line}`);
        return {
          x: parseInt(match[1]),
          y: parseInt(match[2]),
        };
      }
      return {
        x: parseInt(parts[0]),
        y: parseInt(parts[1]),
      };
    });
}

function findPath(walls: Set<string>, gridSize: number): Position[] | null {
  const start: Position = { x: 0, y: 0 };
  const end: Position = { x: gridSize - 1, y: gridSize - 1 };

  if (walls.has(`${start.x},${start.y}`) || walls.has(`${end.x},${end.y}`)) {
    return null;
  }

  const queue: Position[] = [start];
  const visited = new Set<string>([`${start.x},${start.y}`]);
  const parent = new Map<string, Position | null>();
  parent.set(`${start.x},${start.y}`, null);

  const directions = [
    { x: 0, y: 1 },
    { x: 1, y: 0 },
    { x: 0, y: -1 },
    { x: -1, y: 0 },
  ];

  while (queue.length > 0) {
    const current = queue.shift()!;

    if (current.x === end.x && current.y === end.y) {
      // Reconstruct path
      const path: Position[] = [];
      let pos: Position | null = current;
      while (pos !== null) {
        path.unshift(pos);
        const key: string = `${pos.x},${pos.y}`;
        pos = parent.get(key) || null;
      }
      return path;
    }

    for (const dir of directions) {
      const next: Position = {
        x: current.x + dir.x,
        y: current.y + dir.y,
      };

      if (
        next.x < 0 ||
        next.x >= gridSize ||
        next.y < 0 ||
        next.y >= gridSize
      ) {
        continue;
      }

      const key = `${next.x},${next.y}`;
      if (visited.has(key) || walls.has(key)) {
        continue;
      }

      visited.add(key);
      parent.set(key, current);
      queue.push(next);
    }
  }

  return null;
}

function Day18() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [input, setInput] = useState("");
  const [time, setTime] = useState(0);
  const [isPlaying, setIsPlaying] = useState(false);
  const [speed, setSpeed] = useState(10); // frames per second

  // Load input file on mount
  useEffect(() => {
    fetch(`${import.meta.env.VITE_BASENAME || ""}/data/2024/input18.txt`)
      .then((res) => res.text())
      .then((text) => setInput(text))
      .catch((err) => console.error("Failed to load input file:", err));
  }, []);

  // Parse positions
  const positions = useMemo(() => {
    try {
      return parseInput(input);
    } catch (error) {
      console.error("Failed to parse input:", error);
      return [];
    }
  }, [input]);

  const maxTime = positions.length;

  // Calculate current path using BFS
  const currentPath = useMemo(() => {
    if (positions.length === 0) return null;

    // Build walls set from positions up to current time
    const walls = new Set<string>();
    for (let t = 0; t < time && t < positions.length; t++) {
      const { x, y } = positions[t];
      if (x >= 0 && x < GRID_SIZE && y >= 0 && y < GRID_SIZE) {
        walls.add(`${x},${y}`);
      }
    }

    return findPath(walls, GRID_SIZE);
  }, [time, positions]);

  // Draw current frame
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas || positions.length === 0) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    // Clear canvas with dark background
    ctx.fillStyle = "#0f0f23";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // Draw walls up to current time
    // Color scheme: Walls use red/orange gradient, path will use cyan/blue
    for (let t = 0; t < time && t < positions.length; t++) {
      const { x, y } = positions[t];
      if (x < 0 || x >= GRID_SIZE || y < 0 || y >= GRID_SIZE) continue;

      // Color gradient based on age (newer = brighter, older = still clearly visible)
      const age = time - t;
      const maxAge = Math.min(time, 100);
      const ratio = Math.min(1, age / maxAge);

      // Red/Orange gradient for walls
      // Newer walls: bright orange, Older walls: darker red (but still visible)
      const r = Math.floor(255 - ratio * 80);
      const g = Math.floor(150 - ratio * 100);
      const b = Math.floor(30 - ratio * 20);

      ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;
      ctx.fillRect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    }

    // Draw path in bright cyan if it exists
    if (currentPath) {
      ctx.fillStyle = "#00ccff";
      for (const { x, y } of currentPath) {
        ctx.fillRect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE);
      }
    }
  }, [time, positions, currentPath]);

  // Animation loop
  useEffect(() => {
    if (!isPlaying) return;

    const interval = setInterval(() => {
      setTime((t) => {
        if (t >= maxTime) {
          setIsPlaying(false);
          return t;
        }
        return t + 1;
      });
    }, 1000 / speed);

    return () => clearInterval(interval);
  }, [isPlaying, speed, maxTime]);

  return (
    <div style={{ padding: "20px", fontFamily: "monospace", color: "#cccccc" }}>
      <h1>Advent of Code 2024 - Day 18</h1>

      <div style={{ marginBottom: "20px" }}>
        <textarea
          value={input}
          onChange={(e) => setInput(e.target.value)}
          rows={10}
          cols={50}
          style={{
            fontFamily: "monospace",
            backgroundColor: "#1e1e2e",
            color: "#cccccc",
            border: "1px solid #555",
            padding: "8px",
            width: "100%",
          }}
          placeholder="Paste input here (format: x,y)"
        />
      </div>

      <div style={{ marginBottom: "20px" }}>
        <canvas
          ref={canvasRef}
          width={GRID_SIZE * CELL_SIZE}
          height={GRID_SIZE * CELL_SIZE}
          style={{ border: "1px solid #555", display: "block" }}
        />
      </div>

      <div style={{ marginBottom: "10px" }}>
        <label>
          Time:{" "}
          <input
            type="number"
            min={0}
            max={maxTime}
            value={time}
            onChange={(e) => {
              const val = parseInt(e.target.value);
              if (!isNaN(val) && val >= 0 && val <= maxTime) {
                setTime(val);
              }
            }}
            style={{
              width: "80px",
              fontFamily: "monospace",
              backgroundColor: "#1e1e2e",
              color: "#cccccc",
              border: "1px solid #555",
              padding: "4px",
            }}
          />{" "}
          / {maxTime}
          <input
            type="range"
            min={0}
            max={maxTime}
            value={time}
            onChange={(e) => setTime(parseInt(e.target.value))}
            style={{ width: "100%", marginTop: "5px" }}
          />
        </label>
      </div>

      <div style={{ marginBottom: "10px" }}>
        <button
          onClick={() => setIsPlaying(!isPlaying)}
          style={{
            padding: "8px 16px",
            marginRight: "10px",
            cursor: "pointer",
            backgroundColor: "#2e2e3e",
            color: "#cccccc",
            border: "1px solid #555",
          }}
        >
          {isPlaying ? "⏸ Pause" : "▶ Play"}
        </button>
        <button
          onClick={() => setTime(0)}
          style={{
            padding: "8px 16px",
            marginRight: "10px",
            cursor: "pointer",
            backgroundColor: "#2e2e3e",
            color: "#cccccc",
            border: "1px solid #555",
          }}
        >
          ⏮ Reset
        </button>
        <label style={{ marginLeft: "20px" }}>
          Speed: {speed} fps
          <input
            type="range"
            min={1}
            max={60}
            value={speed}
            onChange={(e) => setSpeed(parseInt(e.target.value))}
            style={{ marginLeft: "10px", width: "150px" }}
          />
        </label>
      </div>

      <div style={{ fontSize: "0.9em", color: "#ddd" }}>
        <p>
          Walls placed: {time} / {maxTime}
          {time > 0 && time <= positions.length && (
            <span style={{ marginLeft: "10px", color: "#ffaa00" }}>
              Last wall at: ({positions[time - 1].x},{positions[time - 1].y})
            </span>
          )}
        </p>
        <p>
          Path:{" "}
          {currentPath ? (
            <span style={{ color: "#00ffff" }}>
              Found (length: {currentPath.length - 1} steps)
            </span>
          ) : (
            <span style={{ color: "#ff5555" }}>No path available</span>
          )}
        </p>
      </div>
    </div>
  );
}

export default Day18;
