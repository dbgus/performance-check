# performance check
###  web server framework simple performance check


## check list
1. return plain string (`Hello, world!`)
2. return random number

## check languages
    1. rust(rocket)
    2. TS(nest)

## check command wrk
- wrk -t10 -c100 -d30s "url"

## rust
```
# hello world
Running 30s test @ http://localhost:8000
10 threads and 100 connections
Thread Stats   Avg      Stdev     Max   +/- Stdev
Latency    10.37ms   15.02ms 257.86ms   94.30%
Req/Sec     1.31k   388.85     2.48k    66.00%
390624 requests in 30.03s, 92.39MB read
Requests/sec:  13008.30
Transfer/sec:      3.08MB
```

```
# random
Running 30s test @ http://localhost:8000/rand
  10 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    12.10ms   15.87ms 265.73ms   93.00%
    Req/Sec     1.11k   412.07     2.33k    63.73%
  330394 requests in 30.06s, 74.12MB read
Requests/sec:  10992.33
Transfer/sec:      2.47MB
```

## nest

```
# hello world
Running 30s test @ http://localhost:3005/
  10 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    26.08ms   19.84ms 212.34ms   88.01%
    Req/Sec   433.21    222.30     0.92k    58.87%
  129249 requests in 30.10s, 29.58MB read
Requests/sec:   4294.70
Transfer/sec:      0.98MB
```

```
# random
Running 30s test @ http://localhost:3005/rand
  10 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    23.40ms   15.48ms 159.23ms   88.31%
    Req/Sec   466.17    194.39     2.01k    62.58%
  139157 requests in 30.06s, 32.68MB read
Requests/sec:   4629.99
Transfer/sec:      1.09MB
```