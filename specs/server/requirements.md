<!-- SPDX-License-Identifier: CC0-1.0 -->

# HTTP Server Component — Requirements

## Overview

The HTTP server is an optional, lightweight wrapper that serves the
compiled web-based UI (WASM, HTML, JavaScript, and static assets) over HTTP.
Its sole purpose is to make the single-page application accessible from a
web browser on a host other than the development machine.

The server enables remote observation of the simulation — the development
environment is headless, but a user on a networked machine can access and
interact with the cart-and-pole simulator through their browser. The server
itself contains no simulation logic; all physics and rendering are handled by
the web UI component running as WASM in the browser.

## Behavior / Operator Stories

**As someone running this project:**
- I want to start the HTTP server with a single command so that the web UI is immediately accessible from another host on my network.
- I want to point my browser to an address like `http://<machine>:8080` and have the simulator load and run without additional setup.
- I want the server to serve the built web UI artifacts (WASM binary, HTML, CSS, JavaScript) correctly and without modification.

## Inputs / Outputs

**Conceptual Input:**
- Static web UI files (built WASM binary, HTML, CSS, JavaScript, and any other assets produced by the web UI component)
- A port and/or bind address (may default to `localhost:8080` or similar; operators should be able to override it)

**Conceptual Output:**
- HTTP responses serving the requested static files
- Standard HTTP status codes and error responses for missing files

**File Types:**
- HTML, JavaScript, CSS, WASM binaries, and any image or font assets required by the web UI

## Acceptance Criteria

1. **Static File Serving:** The server correctly serves HTML, CSS, JavaScript, WASM, and other static assets from the web UI build directory in response to HTTP requests.
2. **Content Types:** The server sets appropriate MIME types for different file kinds (e.g., `application/wasm` for `.wasm`, `text/html` for `.html`).
3. **Remote Accessibility:** The web UI is reachable from a browser on a different host when the server is bound to a network-accessible address (not just localhost).
4. **Single-Page Application Support:** The server handles routing for a single-page application, such as serving `index.html` for path requests that don't map to a physical file (enabling client-side routing in the UI).
5. **Minimal Configuration:** The server is runnable with no configuration file required; command-line arguments (or environment variables) are sufficient to specify the bind address and file root.
6. **Error Handling:** Missing or malformed requests receive appropriate HTTP error responses.
7. **Simple Command:** Starting the server requires a single invocation, with no multi-step setup.

## Out of Scope

- **Building or compiling the web UI:** The web component is responsible for producing the static files. This server only consumes them.
- **Simulation logic or physics:** All simulation runs in the browser as WASM; the server is purely a file-serving layer.
- **Dynamic content generation:** No server-side rendering, templating, or API endpoints.
- **Complex routing or business logic:** The server does not implement advanced HTTP features like caching headers, compression, or conditional requests beyond basic HTTP.
- **Authentication, authorization, or security hardening:** This is a learning project component for local/network use; it is not intended for production or public deployment.
- **This component is optional:** Developers or users may choose to serve the
  UI via simpler alternatives (e.g., a static file server like
  `python -m http.server`, `npm serve`, or a reverse proxy) rather than this
  Rust-based server. The requirements describe the minimal feature set for a
  dedicated server component.
