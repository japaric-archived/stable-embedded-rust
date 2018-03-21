.global syscall

syscall:
  bkpt 0xAB
  bx lr
