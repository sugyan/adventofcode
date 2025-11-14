import { useEffect, useRef, useState, useMemo } from "react";

const WIDTH = 101;
const HEIGHT = 103;
const MAX_TIME = WIDTH * HEIGHT;
const CELL_SIZE = 6;

interface Robot {
  px: number;
  py: number;
  vx: number;
  vy: number;
}

function parseInput(input: string): Robot[] {
  return input
    .trim()
    .split("\n")
    .filter((line) => line.trim().length > 0)
    .map((line) => {
      const match = line.match(/p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)/);
      if (!match) throw new Error(`Invalid line: ${line}`);
      return {
        px: parseInt(match[1]),
        py: parseInt(match[2]),
        vx: parseInt(match[3]),
        vy: parseInt(match[4]),
      };
    });
}

function calculatePosition(robot: Robot, t: number): [number, number] {
  const x = (((robot.px + robot.vx * t) % WIDTH) + WIDTH) % WIDTH;
  const y = (((robot.py + robot.vy * t) % HEIGHT) + HEIGHT) % HEIGHT;
  return [x, y];
}

function Day14() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [input, setInput] = useState("");
  const [time, setTime] = useState(0);
  const [isPlaying, setIsPlaying] = useState(false);
  const [speed, setSpeed] = useState(10); // frames per second
  const [pngSize, setPngSize] = useState<number | null>(null);

  // Load input file on mount
  useEffect(() => {
    fetch(`${import.meta.env.VITE_BASENAME || ""}/data/2024/input14.txt`)
      .then((res) => res.text())
      .then((text) => setInput(text))
      .catch((err) => console.error("Failed to load input file:", err));
  }, []);

  // Parse robots and precompute all frames
  const frames = useMemo(() => {
    try {
      const robots = parseInput(input);
      const allFrames: Map<string, number>[] = [];

      for (let t = 0; t <= MAX_TIME; t++) {
        const positions = new Map<string, number>();
        for (const robot of robots) {
          const [x, y] = calculatePosition(robot, t);
          const key = `${x},${y}`;
          positions.set(key, (positions.get(key) || 0) + 1);
        }
        allFrames.push(positions);
      }

      return allFrames;
    } catch (error) {
      console.error("Failed to parse input:", error);
      return [];
    }
  }, [input]);

  // Draw current frame
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas || frames.length === 0) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    // Clear canvas
    ctx.fillStyle = "#0f0f23";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // Draw robots
    const positions = frames[time];
    positions.forEach((count, key) => {
      const [x, y] = key.split(",").map(Number);

      // Color intensity based on count
      const intensity = Math.min(255, 100 + count * 50);
      ctx.fillStyle = `rgb(${intensity}, ${intensity / 2}, 0)`;
      ctx.fillRect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    });

    // Measure PNG size
    canvas.toBlob((blob) => {
      if (blob) {
        setPngSize(blob.size);
      }
    }, "image/png");
  }, [time, frames]);

  // Animation loop
  useEffect(() => {
    if (!isPlaying) return;

    const interval = setInterval(() => {
      setTime((t) => (t + 1) % (MAX_TIME + 1));
    }, 1000 / speed);

    return () => clearInterval(interval);
  }, [isPlaying, speed]);

  return (
    <div style={{ padding: "20px", fontFamily: "monospace", color: "#cccccc" }}>
      <h1>Advent of Code 2024 - Day 14</h1>

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
          placeholder="Paste input here (format: p=x,y v=vx,vy)"
        />
      </div>

      <div style={{ marginBottom: "20px" }}>
        <canvas
          ref={canvasRef}
          width={WIDTH * CELL_SIZE}
          height={HEIGHT * CELL_SIZE}
          style={{ border: "1px solid #555", display: "block" }}
        />
      </div>

      <div style={{ marginBottom: "10px" }}>
        <label>
          Time:{" "}
          <input
            type="number"
            min={0}
            max={MAX_TIME}
            value={time}
            onChange={(e) => {
              const val = parseInt(e.target.value);
              if (!isNaN(val) && val >= 0 && val <= MAX_TIME) {
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
          / {MAX_TIME}
          <input
            type="range"
            min={0}
            max={MAX_TIME}
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
            max={30}
            value={speed}
            onChange={(e) => setSpeed(parseInt(e.target.value))}
            style={{ marginLeft: "10px", width: "150px" }}
          />
        </label>
      </div>

      <div style={{ fontSize: "0.9em", color: "#ddd" }}>
        <p>
          Unique positions: {frames[time]?.size || 0} / Overlapping:{" "}
          {Array.from(frames[time]?.values() || []).filter((count) => count > 1)
            .length || 0}
          {pngSize !== null && (
            <span style={{ marginLeft: "20px" }}>
              PNG size: {(pngSize / 1024).toFixed(2)} KB
            </span>
          )}
        </p>
      </div>
    </div>
  );
}

export default Day14;
