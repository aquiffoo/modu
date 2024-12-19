# Server
Usage: **modu server [port: default 2424]**

This will start the built-in modu interpreter server, which can be used to run code on platforms not support by modu, like web! \
An example of this is the [Modu Web IDE](../ide).

The server has the followings endpoints:
```
GET  /     - Can be used to check if server on
POST /eval - Used to run code
```

In order to run some code, make a post requets to /eval with the code as raw text in the body, like:
```bash
curl --location 'http://localhost:2424/eval' \
     --header 'Content-Type: text/plain' \
     --data 'let a = 1;
     print(a);

     print("YOOOOO");

     if a == 1 {
         print("LES GOOOO");
     }

     print(a);'
```
This will return a response in plaintext like
```
1
YOOOOO
LES GOOOO
1

```