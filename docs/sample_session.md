## What has changed in my branch?
git status

## Worked on formatted print source. Make these known to git and stage for commits.
git add formatted_print/src/*
git add formatted_print/Cargo.*

## Commit when ready.
git commit

## Push changes to remote repo
git push origin master

## Did the changes take place?
git status

## Accidentally committed the binaries! Remove them and recommit!
git rm ./formatted_print/target/* -r
git commit

## Publish to remote
git push origin master

