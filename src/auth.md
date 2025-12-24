# Authorization

In order to use this service, you will need to set the [-l flag](./options.html) along with the token you should have been provided. There are two ways to do this.

## Use as flag
You will need `-l <token>` as part of your command for every command.
```
lbin-cli -l <token> -F ./path/to/file
```

## Set in command line
Or, you can set the following in your terminal or .rc file and not have to use the -l flag as part of the command.
```
export LBIN_AUTH=<token>
```
