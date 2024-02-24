# Rust (Roullie + Postgres[partition])

```
================================================================================
---- Global Information --------------------------------------------------------
> request count                                      61503 (OK=61495  KO=8     )
> min response time                                      1 (OK=1      KO=2     )
> max response time                                   3277 (OK=3277   KO=8     )
> mean response time                                     7 (OK=7      KO=5     )
> std deviation                                         37 (OK=37     KO=2     )
> response time 50th percentile                          3 (OK=3      KO=5     )
> response time 75th percentile                          3 (OK=3      KO=7     )
> response time 95th percentile                         21 (OK=21     KO=8     )
> response time 99th percentile                         97 (OK=97     KO=8     )
> mean requests/sec                                247.996 (OK=247.964 KO=0.032 )
---- Response Time Distribution ------------------------------------------------
> t < 800 ms                                         61483 (100%)
> 800 ms <= t < 1200 ms                                  2 (  0%)
> t >= 1200 ms                                          10 (  0%)
> failed                                                 8 (  0%)
---- Errors --------------------------------------------------------------------
> status.find.in(422), but actually found 200                         5 (62,50%)
> jmesPath(saldo.total).find.is(0), but actually found -8             2 (25,00%)
> jmesPath(saldo.total).find.is(-25), but actually found -22          1 (12,50%)
================================================================================
```
