#!/bin/bash
set -e

# Clean old test results
rm -rf test_repo1 test_repo2

git config user.email "test-runner@example.com"
git config user.name "Test Runner"

mkdir test_repo1
cd test_repo1
git init
cd ..
git clone ./test_repo1 test_repo2
cd test_repo1
printf "hi" > temp
git add temp
git commit -m test_commit
git branch -m main

# Debug info

printf "Parent dir:\n"
ls ..

printf "\nTest repo 1:\n"
ls ../test_repo1

printf "\nTest repo 2:\n"
ls ../test_repo2
