---
title: "Issue"
description: "Doks comes with commands for common tasks."
lead: "Doks comes with commands for common tasks."
date: 2020-10-13T15:21:01+02:00
lastmod: 2020-10-13T15:21:01+02:00
draft: false
images: []
menu:
  docs:
    parent: "prologue"
weight: 130
toc: true
---

{{< alert icon="💡" text="You can change the commands in the scripts section of `./package.json`." />}}

Hi @SainiAditya1, @P-E-P has done a great job in explaning the issue of your PR. You PR was great :) But there are some CI issues, which we have to fix. I found myself fighting with these CI issues. Therefore, it's worth mentioning the fixes that I use, hope you found it helpful too :DD

If you see the checks which are failing, there are:
- [x] Merge Commits
- [x] Clang Format Lint / clang-format
- [x]  GNU Commit Format Checker / check-changelogs

---
## Merge Commit Issue:
@P-E-P has explained this issue well. But if go to https://github.com/Rust-GCC/gccrs/pull/2346/commits . You can see that there are indeed merge commits


 ![commits](https://github.com/Rust-GCC/gccrs/assets/95980383/402e61f3-1eb6-411b-8105-009b36b70ea0)

-  For fixing this, you can follow the solution of @P-E-P, or you can use my solution.
```bash
git reset --hard HEAD~4
git reset --soft  HEAD~1
# These comments are for explanation:


# ➜  gccrs git:(Aditiya) git reset --hard HEAD~4
# HEAD is now at 22a269a2260 Move ast collector template implementation to header file
# ➜  gccrs git:(Aditiya) git reset --soft  HEAD~1



# ➜  gccrs git:(Aditiya) ✗ git status              
# On branch Aditiya
# Your branch is behind 'Aditiya/master' by 97 commits, and can be fast-forwarded.
#   (use "git pull" to update your local branch)
# 
# Changes to be committed:
#   (use "git restore --staged <file>..." to unstage)
# 	modified:   gcc/rust/ast/rust-ast-collector.cc
# 	modified:   gcc/rust/ast/rust-ast-collector.h
# 
```

---
## Clang formating issues:
- GCCRS follows the specfic coding format to keep the code format consistent. For this you have to use `clang-format10`.
- On ubuntu 22.04, unfortunatley, I tried downloading clang-format10 from ubuntu main / universe repositories. But, it didn't find that.
- You can download the `clang-format10` binary from [`here`](https://github.com/DoozyX/clang-format-lint-action/blob/master/clang-format/clang-format10)
- Now, run this command to give executable premission to `clang-format10` . You can place this file to the parent of your gccrs folder. So, you can use this like ../clang-format10. Or Alternatively, you can move this file to bin of your linux distro (if you are using linux)
```bash
chmod +x clang-format10
```
Now, you have to run this onto your modified files, in your case, you have to use
```bash
# clang-format10 -i <relative or absolute location of your file>
# If you placed clang-format10 in the parent directory of gccrs local folder,
# And if your in gccrs folder / directory, then you can use
../clang-format10 -i gcc/rust/ast/rust-ast-collector.cc
../clang-format10 -i gcc/rust/ast/rust-ast-collector.h  
# Here, -i is the inplace flag
```

---

### Now, commit format checker issue:
- For keep tracking the changes made in each change gcc uses change logs, to easily view the bird's eye view of the changes in the desired commit.
- You can run this command

```bash
git gcc-commit-mklog
# ➜  gccrs git:(Aditiya) ✗ git gcc-commit-mklog

``` 
- After, this you can see the output like this:
```bash
 GNU nano 6.2                                                 /home/mahad/Desktop/mahad/gccrs/.git/COMMIT_EDITMSG *                                                         
<your message>

gcc/rust/ChangeLog:

        * ast/rust-ast-collector.cc (TokenCollector::visit): <your message>
        (TokenCollector::visit_items_joined_by_separator): <your message>
        (TokenCollector::visit_as_line): <your changes>
        (TokenCollector::visit_items_as_lines): <your message>
        (TokenCollector::visit_items_as_block): <your message>
        * ast/rust-ast-collector.h: <your message>

# Please enter the commit message for your changes. Lines starting
# with '#' will be ignored, and an empty message aborts the commit.
#
# On branch Aditiya
# Your branch is behind 'Aditiya/master' by 97 commits, and can be fast-forwarded.
#   (use "git pull" to update your local branch)
#
# Changes to be committed:
#       modified:   gcc/rust/ast/rust-ast-collector.cc
#       modified:   gcc/rust/ast/rust-ast-collector.h
#
```
- This will open a in nano, then just press `ctrl + O`, then `ctrl + enter` and finally `ctrl + X` for saving that file. 

- If `git gcc-commit-mklog` gives you error then you have to first set.
- you can run the `./contrib/gcc-git-customization.sh` script (for this you may have to pip3 or other python git packages installed on your system), which will add some useful `git` commands like `git gcc-commit-mklog`. It works exactly the same as `gcc commit`
- Moreover, if you'd like to run changelogs checks that check locally, you can run `contrib/gcc-changelog/git_check_commit.py upstream/master..HEAD` in your terminal. this will check that all the commits on your branch (`HEAD`) that aren't yet merged (so not yet on `upstream/master`) respect the Changelog format.
- Moreover, One thing I would suggest is not creating PRs from your master branch. Here's a little article with commands to avoid it: https://blog.jasonmeridth.com/posts/do-not-issue-pull-requests-from-your-master-branch/ (suggested from @CohenArthur)

- Now, you have to sign that commit, (if you are using `gitkraken`, then `gitkraken` will automatically trim new line from the end, which will again give you CI error, therefore, it is better to handle this task to `git`), you can do it manually but git do this for you automatically. By running
```bash
git commit --amend -s  
```
- Now, you can follow the same steps for saving the file. 
-  After this if you run this check `contrib/gcc-changelog/git_check_commit.py upstream/master..HEAD`, it will give you output like this:
```
➜  gccrs git:(Aditiya) contrib/gcc-changelog/git_check_commit.py
Checking f2e9a6726faa4b67442f0dfa1578ee69cdb10dee: OK
```

---

Now, just push your changes.
```bash
git push -f
```
