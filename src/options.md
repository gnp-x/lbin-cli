# Options

```
Usage: lbin-cli -l <LBIN_AUTH> [OPTIONS] <INPUT>...
Arguments:
  <INPUT>...  INPUT
```

|Option|Description|
|------|-----------|
| -l, - -lbin-auth [LBIN_AUTH]|Not required if you `export LBIN_AUTH=<token>`|
|  -f, - -file          |         Upload a file|
|  -i, - -std-input|              Command-line input to upload|
|  -t, - -time [TIME]|            How many minutes until file expires|
|-o, --oneshot|                Upload a file that can only be seen once|
|  -h, - -help    |               Print help|
|  -V, - -version|                Print version|
