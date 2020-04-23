# boxed
Put command output in a box
#Output
```
 % exa -l --sort newest | boxed -c \*
*************************************************
* .rw-r--r-- 11k nirmoy 23 Apr  1:23 LICENSE    *
* drwxr-xr-x   - nirmoy 23 Apr 13:33 target     *
* .rw-r--r-- 236 nirmoy 23 Apr 19:07 Cargo.toml *
* .rw-r--r-- 593 nirmoy 23 Apr 19:08 Cargo.lock *
* drwxr-xr-x   - nirmoy 23 Apr 20:28 src        *
* .rw-r--r-- 943 nirmoy 23 Apr 20:30 README.md  *
*************************************************
 % exa -l --sort newest | boxed -c \!
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
! .rw-r--r-- 11k nirmoy 23 Apr  1:23 LICENSE    !
! drwxr-xr-x   - nirmoy 23 Apr 13:33 target     !
! .rw-r--r-- 236 nirmoy 23 Apr 19:07 Cargo.toml !
! .rw-r--r-- 593 nirmoy 23 Apr 19:08 Cargo.lock !
! drwxr-xr-x   - nirmoy 23 Apr 20:28 src        !
! .rw-r--r-- 943 nirmoy 23 Apr 20:30 README.md  !
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
 % exa -l --sort newest | boxed
╔═══════════════════════════════════════════════╗
║ .rw-r--r-- 11k nirmoy 23 Apr  1:23 LICENSE    ║
║ drwxr-xr-x   - nirmoy 23 Apr 13:33 target     ║
║ .rw-r--r-- 236 nirmoy 23 Apr 19:07 Cargo.toml ║
║ .rw-r--r-- 593 nirmoy 23 Apr 19:08 Cargo.lock ║
║ drwxr-xr-x   - nirmoy 23 Apr 20:28 src        ║
║ .rw-r--r-- 943 nirmoy 23 Apr 20:30 README.md  ║
╚═══════════════════════════════════════════════╝
```
