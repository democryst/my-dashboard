# ADR-003: Frontend Rendering (Leptos + WebGPU Charting)

## Status
Proposed (Stage 2: Architecture Design)

## Context
"LogStream Lens" requires a drag-and-drop interface for real-time visualization of 1GB+ datasets (potentially millions of rows). Standard SVG or DOM-based charting will degrade significantly under this load.

## Decision
We will implement the frontend using **Leptos (Rust/WASM)** and utilize the **`leptos_helios`** library for high-performance charting.

1. **Hardware Acceleration**: `leptos_helios` utilizes **WebGPU** (with Canvas2D fallback) to offload chart rendering to the GPU, enabling fluid interaction even with massive datasets.
2. **Fine-Grained Reactivity**: Leptos's signal-based reactivity ensures that only the affected dashboard panels re-render during cross-filtering, maintaining sub-200ms interactivity.
3. **Rust-Native**: Entirely eliminates the JS-bridge overhead for data-heavy analytics, keeping full type safety from the backend to the UI.

## Consequences
- **Browser Compatibility**: WebGPU requires modern browsers (Chrome/Edge 113+, Safari 17+); the Canvas2D fallback will be used for older systems with lower performance.
- **Learnability**: Requires familiarity with Rust-WASM paradigms and Leptos signals.
