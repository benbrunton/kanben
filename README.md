# Kanben

Kanban for Ben, by Ben. A personal, no-frills kanban tool
for the terminal.


## Usage

```
kanben                      # lists top priority work
kanben add <title>          # creates a new work item
kanban edit <title>         # allows editing task description
kanben view <title>         # lists all info in <title>
kanben start <title>        # moves <title> into doing
kanben complete <title>     # moves <title> into done
kanben delete <title>       # individual delete
kanben clear-done           # clears done column
kanben now                  # outputs in-progress tasks
kanben tag <title> <tag>    # adds a tag to a task
kanben top <title>          # move task to top of list
```
## Install

### with Cargo

`cargo install kanben`

## Autocomplete
`source autocomplete.sh` to enable autocomplete

