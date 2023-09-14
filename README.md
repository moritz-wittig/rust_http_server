# rust_http_server

## to test
start the "server" with `cargo run`.

### with netcat
run `echo "TEST" | netcat 127.0.0.1 8080` in a second terminal to push a String to localhost.

### with the browser
open `http://localhost:8080/` in the browser of your choice. The incoming request will be handled by the server.