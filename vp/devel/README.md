# Renode Platform Description files (.repl)

The contents of this directory specify the Headsail virtual prototype implemented by
Renode. The initial version was generated using Kactus2 (sa.
`../kactus2-generated-*`), and adapted for use (this directory).

The VP is structured as follows:

| Path         | Purpose |
| :-           | :-      |
| common.repl  | Devices available in the unified memory map available to all initiators |
| hpc.repl     | Devices available to HPC as initiator only |
| sysctrl.repl | Devices available to SysCtrl as initiator only |

