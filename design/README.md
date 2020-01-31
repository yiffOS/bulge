# Bulge's design

LAST UPDATE: 2020-01-29

This document will desbcribe how bulge will work.  
Both the actual package manager and the repositories's layout will be defined here.  
Design subject to change.

PACKAGE MANAGER
----

SERVER
----
All repo metadata will be grabbed from the root of the arch folder as JSON data.  
`$server_url/$os_arch/metadata`

The repo metadata format will be as follows:   
```json
{
  "repoInformation": {
    "name": "yiffOS Base Repo",
     "maintainer": {
        "name": "Skye Viau",
        "email": "skye.viau@gmail.com"
     },
    "arch": "x86-64",
    "lastUpdated": "",
  }
}
```