# Rust (HAProxy + Axum + Postgres[partition+function])


Result:
```
================================================================================
---- Global Information --------------------------------------------------------
> request count                                      61503 (OK=61500  KO=3     )
> min response time                                      1 (OK=1      KO=7     )
> max response time                                   2335 (OK=2335   KO=8     )
> mean response time                                    16 (OK=16     KO=7     )
> std deviation                                         64 (OK=64     KO=0     )
> response time 50th percentile                          2 (OK=2      KO=7     )
> response time 75th percentile                          3 (OK=3      KO=8     )
> response time 95th percentile                         91 (OK=91     KO=8     )
> response time 99th percentile                        288 (OK=288    KO=8     )
> mean requests/sec                                251.033 (OK=251.02 KO=0.012 )
---- Response Time Distribution ------------------------------------------------
> t < 800 ms                                         61461 (100%)
> 800 ms <= t < 1200 ms                                  8 (  0%)
> t >= 1200 ms                                          31 (  0%)
> failed                                                 3 (  0%)
---- Errors --------------------------------------------------------------------
> jmesPath(saldo.total).find.is(0), but actually found -3             2 (66,67%)
> jmesPath(saldo.total).find.is(-25), but actually found -11          1 (33,33%)
================================================================================
```
