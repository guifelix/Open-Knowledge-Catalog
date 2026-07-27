# strictly follow the process

1. read thorotughtly and acknowledge this process
2. execute `backlog task list --plain`
3. pick one (higher priority) task (as long as it doesn't have dependencies; you dont need to assess all tasks in depth)
4. execute `backlog task view <task id> --plain` and analyze it
5. transition task to "In Progress" (using cli)
6. senior maang-level TDD the task (always make sure to re-read and follow [AGENTS.md](../AGENTS.md))
7. re-read the ACs, DoD and implementation plan to see if there is any missing spot or any gaps
8. check pipeline if its passing (if not go back to step 6)
9. check [docs/](../docs/) and [README.md](../README.md) to see if we need to update docs
10. transition task to done (using cli)
11. use conventional and atomic commits to commit all files (including the task file)


## observation:
- you dont need to `cd` into the project folder to run commands
- always read a file before updating/changing so as not to introduce errors/issues/bugs
- you can use your judment if we'll need to break task into smaller pieces, just follow [backlog-draft-workflow](backlog-draft-workflow.md) to create drafts
- as a senior maang-level employee, feel free to
  - delegate work to your agents
  - question any aspect of the task