## Project: Aero-MDX (Agent-Operated Market Data & Execution Gateway)

To bridge your background in fintech data engineering with this ultra-low-latency, Rust-heavy trading role, you need a project that proves you can write highly concurrent systems while satisfying Keyrock's unique requirement for **Agentic Engineering**.

Aero-MDX is a lightweight, zero-copy crypto market data gateway and simulated execution engine, heavily instrumented for AI-driven operational management via an MCP (Model Context Protocol) server.

### Core Architecture

This project targets the exact intersection of the job description: Tokio-based Rust concurrency, low-latency data handling, cloud deployment, and LLM-assisted workflow automation.

#### 1. The Low-Latency Rust Core

* **Zero-Copy Ingestion:** A Tokio-based WebSocket client connecting to a live crypto exchange (e.g., Binance or Kraken). Use `simd-json` or strict `serde` with zero-copy deserialization (`&'a str`) to parse real-time L2 order book updates.
* **Lock-Free Concurrency:** Pin your ingestion thread and your order-book-building thread to specific CPU cores using `core_affinity`. Pass parsed tick data between them using a lock-free ring buffer (e.g., `crossbeam` or `rtrb`) to demonstrate understanding of kernel bypass and context-switch minimization.
* **Mock FIX Execution Engine:** A simple TCP server simulating a FIX (Financial Information eXchange) gateway. It accepts incoming mock orders and crosses them against your locally maintained L2 order book.

#### 2. The "Agentic" Ops Layer (The Standout Feature)

* **System Telemetry via Shared Memory:** The Rust engine writes its performance metrics (tick-to-trade latency, memory usage, order book depth, dropped packets) to a memory-mapped file (`mmap`) or a fast local Unix domain socket.
* **MCP Server (Model Context Protocol):** Write a lightweight Python or Rust MCP server that exposes these metrics as "tools" to an LLM (like Claude Desktop or Cursor).
* **AI Auto-Remediation:** Create a prompt workflow where you can ask your AI IDE, *"Why did our 99th percentile latency spike at 10:04 AM?"* The AI uses the MCP tools to query the gateway's logs, identifies a garbage collection pause or network jitter, and suggests a kernel parameter fix.

#### 3. Cloud & Infrastructure (AWS & Linux)

* **Infrastructure as Code:** Write a Terraform or AWS CDK script to deploy this onto an AWS EC2 `c7g` (Graviton/ARM) instance to show cloud fluency.
* **Linux Tuning Script:** Include a bash script that tunes the Linux kernel for latency before launching the app. Include configurations for isolating CPU cores (`isolcpus`), disabling hyperthreading, and tuning the TCP stack (e.g., `tcp_low_latency`, disabling Nagle's algorithm).

### Implementation Roadmap

**Phase 1: Market Data Pipeline (Proves Rust/Tokio)**
Build the WebSocket ingestion and L2 order book compiler. Focus purely on memory allocation (use arena allocators or object pools to avoid runtime allocation overhead). Benchmark your parsing latency (aim for microseconds).

**Phase 2: Execution Simulator (Proves Trading/Networking)**
Implement a basic TCP listener that accepts a mock order and returns a fill based on your L2 book. Log the internal latency from order-received to fill-dispatched.

**Phase 3: The Agentic Harness (Proves the "Keyrock" Factor)**
Build the MCP server. This proves you read the JD deeply. Keyrock specifically asks for MCP API experience and AI workflow automation. Showing you can build an AI agent that monitors and queries your trading engine will put you ahead of candidates who just built a standard trading bot.

**Phase 4: AWS & Packaging (Proves Production Readiness)**
Wrap it in a Docker container (though note that in real HFT, containers add overhead—mention this in your README) and write the Terraform deployment.
