Write a rust CLI, this cli will accept a file path as an argument. The default value is `C:\PkgCache\VC17LTCG`.
The file path is the cache folder for packages. Each subfolder is a package, and each package folder contains many versions of the package.
The CLI should clean all packages and only keep the latest 2 versions of each package.