# Examples
## File
### Regular
- This hosts the file for 6 hours.
```
lbin -f /path/to/file
```
### One-Shot File 
- Same as above, but the link is one-time use only.
```
lbin -F ./path/to/file
```

## URL
### Shortener
- Good for atrociously long urls.
```
lbin -u https://some-really-long-url-that-may-be-really-annoying-to-paste-somewhere-.com
```
### One-Shot URL
- Same as above, but the link is one-time use only.
```
lbin -U https://liminal.cafe
```
### Remote File
- Link a file from a different site.

***Note:** This may not work with some websites that discourage hotlinking.*
```
lbin -r https://example.com/some-file.jpeg
```

## Command-Line Input
- Good if you just need to do a quick paste from the terminal and can't be bothered to package it into a file. This command essentially does that for you!
```
lbin -i This text will be available in an uploaded .txt file to read.
```
