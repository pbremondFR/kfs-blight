file kfs.elf
target remote localhost:1234
define forcequit
  kill
  quit
end
alias fq = forcequit
