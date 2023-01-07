# Recho v1.0.0
---
---
#### Summary:
A GNU UNIX style "echo" command line utility with added Base64 functionality. Recho offers all the functionality of 
traditional ECHO, but allows the user to encode/decode strings into/from Base64. 
---
#### Usage:
Recho has three switches that can be utilized from the command line:

|                  **Switch**                   |              **Description**               |
|:---------------------------------------------:|:------------------------------------------:|
|                     `-h`                      |          to display runtime help           |
|                     `-tb`                     |  to convert the passed string "to-base64"  |
|`-fb`| to convert the passed string "from-base64" |
---
#### Examples:
The following are the example uses for Recho:

```bash
# By default does include newlines.
> recho.exe "Hello World!"
Hello world!

# To remove newlines after the string, pass the -n switch
> recho.exe -n "Hello World!"
Hello world!
# To encode something in base64, pass the -tb switch, which also removes newline character
> recho.exe -tb "Hello World!"
SGVsbG8gd29ybGQh
# To decode something in base64, pass the -fb switch, which also removes the newline character.
> recho.exe -fb "SGVsbG8gd29ybGQh"
Hello World!
# To display the inline help scree, pass the -h switch.
recho: GNU ECHO re-written in Rust with added functionality
Usage and Examples:
Switches:
    -h : help, displays this
    -n : no new line character added at the end
    -tb : to-base64, encodes the string in base64, automatically passes the (n) switch as well.
    -fb : from-base64, decodes the string from base64, automatically passes the (n) switch as well.
Usage:
    NOTE: Strings containing spaces need to be wrapped in quotes.
    recho.exe <SWITCHES> <TEXT>
    recho.exe -n 'hello world'
    recho.exe 'hello world'
    recho.exe -tb 'hello world' (Will print out aGVsbG8gd29ybGQ=)
```