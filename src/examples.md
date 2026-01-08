# Examples
## File
- This hosts the file for 6 hours.
```
lbin-cli -f /path/to/file
```

## Command-Line Input
- Good if you just need to do a quick paste from the terminal and can't be bothered to package it into a file. This command essentially does that for you!
```
lbin-cli -i This text will be available in an uploaded .txt file to read.
```

## Expiry
- This hosts the file for 5 minutes.
```
lbin-cli -f /path/to/file -t 5
```
- This hosts the file for 2 hours.
```
lbin-cli -i "Hello there." -t 120
```
## One-shot
- This deletes the file once visited.
```
lbin-cli -o -f /path/to/file
```
```
lbin-cli -o -i "Hello there."
```
