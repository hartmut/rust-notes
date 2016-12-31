#!/bin/bash
cd ..
echo "blog.rust-lang.org"
cd blog.rust-lang.org
git pull
echo "rust-wiki-backup"
cd ../rust-wiki-backup
git pull
echo "rust book"
cd ../book
git pull
echo "rust itself"
cd ../rust
git pull
echo "rust-rfcs"
cd ../rust-rfcs
git pull
echo "rust-by-example"
cd ../rust-by-example
git pull
#echo "get documentations"
#cd ../rust-doc
#wget -rN https://doc.rust-lang.org/

