Prexus Connect is a next-generation, lightweight API gateway built in Rust. It embodies the "Shared Responsibility Model" in distributed systems and is designed to protect both vendors and clients from system failures and legal risks.

Serving as the "entry point (freemium)" to the Prexus SaaS ecosystem, this OSS edition provides an environment where developers can easily conduct negative testing, helping to reduce debugging efforts and improve system robustness. With an in-memory design that completely eliminates database (DB) dependencies, no tedious setup is required. Anyone can launch it in seconds with a single command.

✨ Core Features

Prexus Crucible (Chaos Engineering Mock)
Intentionally triggers "anomalies" that are difficult to avoid in standard web development, allowing you to test your application's error handling and fault tolerance before deployment.
Timeout Simulation: Replicates communication spikes and delays (5 seconds).
Conflict Simulation: Replicates data overwrite conflicts (HTTP 409).
Data Loss Simulation: Replicates physical server/DB failures (HTTP 500).

Sentinel Audit Logger (Compliance Logger)
Provides intelligent logging that automatically records all communications and prevents the leakage of confidential information.
Auto-Masking: Detects sensitive information such as email addresses (strings containing @, etc.) in the URL path, automatically replaces them with masked_path, and records them securely.
Performance Tracking: Measures latency for each request in milliseconds.

Billing & Rate Limiter (Multi-Layered Cap / Failsafe)
An auto-throttling feature that protects your system and finances from abnormal external traffic and infinite loops caused by bugs.
Tenant Isolation: Individually aggregates usage per tenant using the x-tenant-id header.
Safety Valve: When excessive requests (exceeding the limit) are detected, it blocks communication at the physical level without performing subsequent processing.

🛠️ Quick Start

Prerequisites
Rust / Cargo (Latest stable version recommended)

Installation and Startup

Clone the repository
git clone https://github.com/your-username/prexus-connect-oss.git
cd prexus-connect-oss

Start the server
cargo run

If 🚀 Prexus Connect (OSS) started on port 8080... is displayed, you are ready to go.

🧪 Usage Examples

Open a new terminal and use the following commands to check the behavior of the API gateway.

Normal Connectivity Check
curl -v -X POST http://localhost:8080/api/crucible/test

Timeout Simulation
curl -v -X POST http://localhost:8080/api/crucible/test -H "x-crucible-scenario: timeout"

Data Conflict (Conflict 409) Simulation
curl -v -X POST http://localhost:8080/api/crucible/test -H "x-crucible-scenario: conflict"

Auto-Masking Audit Log Check
(Recorded as masked_path in the standard output on the server side)
curl -v -X GET "http://localhost:8080/api/mock/generic?user=test@example.com"

💡 Frequently Asked Questions (Q&A)

Q. Can this OSS only be used to test Rust applications?
A. No, the language of the target application "does not matter at all".
Because Prexus Connect operates as an independent API gateway (Web server), it can be placed in front of applications developed in any language or framework—such as Python, Node.js, PHP, Ruby, Go, or Java—for testing and integration.

🔮 Roadmap: The Prexus Ecosystem

This Prexus Connect (OSS Edition) is merely the entrance to the "Prexus Ecosystem" we envision. This OSS version allows you to experience a portion of the "insurance value" proposed by Prexus for free.

In the future, we plan to provide enterprise-grade features to meet more advanced requirements.

Crucible Pro: An advanced test scenario expansion pack of medical and financial grade.
Crucible Verified: Issuance of an official robustness certificate for solid systems that have passed the tests.
The Vault: Integration of an absolutely secure data vault with legal evidentiary capacity, not allowing a single character to be tampered with.

First, try using this OSS gateway to push your application's robustness and compliance to the absolute limit.

📄 License

This project is released under the MIT License.

Developed by Akio Akasaka (Prexus Founder)
Empowering Trust through Resilient Architecture.