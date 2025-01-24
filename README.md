# m-o

Metrics and OTEL

```shell
➜  m-o git:(main) ✗ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/m-o`
Metrics
Resource
	 ->  service.name=String(Owned("m-o"))
	Instrumentation Scope #0
		Name         : actix-web-opentelemetry
		Version  : "0.20.1"
		SchemaUrl: "https://opentelemetry.io/schemas/1.28.0"
Metric #0
		Name         : http.server.duration
		Description  : Measures the duration of inbound HTTP requests.
		Unit         : s
		Type         : Histogram
		Temporality  : Cumulative
		Histogram DataPoints
		DataPoint #0
			StartTime    : 2025-01-24 14:07:10.140051
			EndTime      : 2025-01-24 14:08:10.140579
			Count        : 11
			Sum          : 19.837263478000004
			Min          : 1.802292795
			Max          : 1.804464909
			Attributes   :
				 ->  http.route: /
				 ->  http.request.method: GET
				 ->  network.protocol.version: 1.1
				 ->  server.address: localhost
				 ->  server.port: 8080
				 ->  url.scheme: http
				 ->  http.response.status_code: 200
Metric #1
		Name         : http.server.active_requests
		Description  : Measures the number of concurrent HTTP requests that are currently in-flight.
		Unit         :
		Type         : Sum
		Sum DataPoints
		Monotonic    : false
		Temporality  : Cumulative
		DataPoint #0
			StartTime    : 2025-01-24 14:07:10.140114
			EndTime      : 2025-01-24 14:08:10.140644
			Value        : 0
			Attributes   :
				 ->  http.request.method: GET
				 ->  http.route: /
				 ->  network.protocol.version: 1.1
				 ->  server.address: localhost
				 ->  server.port: 8080
				 ->  url.scheme: http
Metric #2
		Name         : http.server.request.size
		Description  : Measures the size of HTTP request messages (compressed).
		Unit         : By
		Type         : Histogram
		Temporality  : Cumulative
		Histogram DataPoints
		DataPoint #0
			StartTime    : 2025-01-24 14:07:10.140175
			EndTime      : 2025-01-24 14:08:10.140685
			Count        : 11
			Sum          : 0
			Min          : 0
			Max          : 0
			Attributes   :
				 ->  http.request.method: GET
				 ->  http.route: /
				 ->  network.protocol.version: 1.1
				 ->  server.address: localhost
				 ->  server.port: 8080
				 ->  url.scheme: http
Metric #3
		Name         : http.server.response.size
		Description  : Measures the size of HTTP response messages (compressed).
		Unit         : By
		Type         : Histogram
		Temporality  : Cumulative
		Histogram DataPoints
		DataPoint #0
			StartTime    : 2025-01-24 14:07:10.140219
			EndTime      : 2025-01-24 14:08:10.140719
			Count        : 11
			Sum          : 55
			Min          : 5
			Max          : 5
			Attributes   :
				 ->  http.route: /
				 ->  http.request.method: GET
				 ->  network.protocol.version: 1.1
				 ->  server.address: localhost
				 ->  server.port: 8080
				 ->  url.scheme: http
				 ->  http.response.status_code: 200
```
