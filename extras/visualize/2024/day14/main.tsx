import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './../../src/index.css'

function Day01() {
  return (
    <div>
      <h1>Advent of Code 2024 - Day 01</h1>
      <p>Visualization goes here</p>
    </div>
  )
}

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <Day01 />
  </StrictMode>,
)
