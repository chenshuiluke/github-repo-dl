#Usage

    github-repo-dl <token>

#Useful Commands

##Removing all files except source files

    shopt -s extglob ; sudo rm -r !(Cargo.lock|Cargo.toml|src|target|README.md)
