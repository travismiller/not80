# Not 80

An exploration of Rust & Hyper. Redirect all HTTP traffic to HTTPS.

## Usage

```console
$ LISTEN=0.0.0.0:80 cargo run
```

```console
$ curl -v 'http://localhost/'
*   Trying ::1...
* connect to ::1 port 80 failed: Connection refused
*   Trying 127.0.0.1...
* Connected to localhost (127.0.0.1) port 80 (#0)
> GET / HTTP/1.1
> Host: localhost
> User-Agent: curl/7.43.0
> Accept: */*
>
< HTTP/1.1 302 Found
< Location: https://localhost/
< Content-Length: 89
< Content-Type: text/html
< Date: Wed, 27 Dec 2017 23:19:24 GMT
<
<!doctype>
<html><body><a href="https://localhost/">https://localhost/</a></body></html>
* Connection #0 to host localhost left intact
```
