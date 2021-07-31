
# Notes

* on VPS, before build: ```sudo chmod 777 qazvin_bus_status_ms/```
* ```cd qazvin_bus_status_ms && cargo install cargo-watch```


# Example
[http://localhost:7366/avl/api/reports/status/166/2021-04-12T00:00:00+06:00/2021-05-12T23:59:59+06:00]()


# Setup

* **Watch _reports_ microservice:** ```cargo watch -x 'run --bin reports'```
* **Build the service:** ```cargo build --bin reports --release```
