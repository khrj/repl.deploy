set -e

# Clean old test results
rm -rf test_repo1 test_repo2

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