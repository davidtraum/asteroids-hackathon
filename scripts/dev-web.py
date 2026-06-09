#!/usr/bin/env python3
from http import HTTPStatus
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from queue import Queue
from subprocess import run
from threading import Event, Thread
from time import sleep
import argparse
import os


PROJECT_ROOT = Path(__file__).resolve().parent.parent
WEB_ROOT = PROJECT_ROOT / "web"
WATCH_PATHS = [
    PROJECT_ROOT / "src",
    PROJECT_ROOT / "Cargo.toml",
    PROJECT_ROOT / "Cargo.lock",
    PROJECT_ROOT / ".cargo" / "config.toml",
]

reload_clients = set()


def iter_watch_files():
    for path in WATCH_PATHS:
        if path.is_file():
            yield path
        elif path.is_dir():
            for root, dirs, files in os.walk(path):
                dirs[:] = [name for name in dirs if not name.startswith(".")]
                for filename in files:
                    yield Path(root) / filename


def snapshot():
    state = {}
    for path in iter_watch_files():
        try:
            stat = path.stat()
        except FileNotFoundError:
            continue
        state[path] = (stat.st_mtime_ns, stat.st_size)
    return state


def build_wasm():
    print("Building WebAssembly...")
    result = run(["bash", "scripts/build-web.sh"], cwd=PROJECT_ROOT)
    if result.returncode == 0:
        print("Build finished. Reloading browser.")
        notify_reload_clients()
    else:
        print("Build failed. Fix the Rust error; the watcher will retry on the next save.")


def notify_reload_clients():
    stale_clients = []
    for client in list(reload_clients):
        try:
            client.put_nowait("reload")
        except Exception:
            stale_clients.append(client)

    for client in stale_clients:
        reload_clients.discard(client)


def watch_files(stop_event):
    previous = snapshot()
    pending = False

    while not stop_event.is_set():
        sleep(0.5)
        current = snapshot()
        if current != previous:
            previous = current
            pending = True
            continue

        if pending:
            pending = False
            build_wasm()


class DevHandler(SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=WEB_ROOT, **kwargs)

    def end_headers(self):
        self.send_header("Cache-Control", "no-store")
        super().end_headers()

    def do_GET(self):
        if self.path == "/__reload":
            self.send_response(HTTPStatus.OK)
            self.send_header("Content-Type", "text/event-stream")
            self.send_header("Cache-Control", "no-cache")
            self.send_header("Connection", "keep-alive")
            self.end_headers()
            self.wfile.write(b": connected\n\n")
            self.wfile.flush()

            events = Queue()
            reload_clients.add(events)
            try:
                while True:
                    event = events.get()
                    self.wfile.write(f"event: {event}\ndata: now\n\n".encode())
                    self.wfile.flush()
            except (BrokenPipeError, ConnectionResetError):
                pass
            finally:
                reload_clients.discard(events)
            return

        super().do_GET()

    def log_message(self, format, *args):
        print(f"{self.address_string()} - {format % args}")


def main():
    parser = argparse.ArgumentParser(description="Serve the Macroquad WASM app with reload-on-build.")
    parser.add_argument("--host", default="127.0.0.1")
    parser.add_argument("--port", default=8000, type=int)
    args = parser.parse_args()

    build_wasm()

    stop_event = Event()
    watcher = Thread(target=watch_files, args=(stop_event,), daemon=True)
    watcher.start()

    server = ThreadingHTTPServer((args.host, args.port), DevHandler)
    print(f"Serving hot-reload web build at http://{args.host}:{args.port}")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        stop_event.set()
        server.server_close()


if __name__ == "__main__":
    main()
