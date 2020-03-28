# Bulge's design

LAST UPDATE: 2020-03-28

This document will describe how bulge will work.  
Both the actual package manager and the repositories's layout will be defined here.  
Design subject to change.

## SERVER

All repo metadata will be grabbed from the root of the arch folder as JSON data.  
`$server_url/$os_arch/metadata.json`

The repo metadata format will be as follows:   
```json
{
  "name": "yiffOS Base Repo",
  "maintainer": {
     "name": "Skye Viau",
     "email": "skye.viau@gmail.com"
  },
  "arch": "x86-64",
  "lastUpdated": "1585394210"
}
```

Each package will have it's own JSON data, the client searches for this data by searching for the package name as a 
folder.   
`$server_url/$os_arch/$package_name/metadata.json`

The metadata for the package looks something like this:
```json
{
  "name": "test-package",
  "version": "1.0.0-dev",
  "summary": "A test Bulge package",
  "homepage": "https://example.com",
  "license": "GPLv3",
  "category": ["development", "testing"],
  "maintainers": ["example@example.com", "example2@example.com"],
  "source": [
    "https://example.com/test-package.tar.gz",
    "https://example2.com/test-package-patch.diff"
  ]
}
```

The build script which is located next to the metadata JSON will be run by the client.
`$server_url/$os_arch/$package_name/build`

The actual build script looks something like this:
```
[prepare]
tar -xzfv $src0
patch $temp/test-package/src/lib/test/main.rs < $src1

[build]
cd $temp/test-package/
./configure
make

[install]
make install
```

`$src` is the source array object   
`$temp` is the folder which the build commands run in



## CLIENT

All packages and a stored local version are kept in a single database structured like this:

```json
{
  "lastUpdated": "1585394210",
  "packages": {
    "test-package": {
      "version": "1.0.0-dev"
    },
    "test-package-2": {
      "version": "1.2"
    } 
  }
}
```