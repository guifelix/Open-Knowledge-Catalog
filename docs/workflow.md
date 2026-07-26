- strictly follow the process
  - read thorotughtly and acknowledge this process
  - execute `backlog task list --plain`
  - pick one (higher priority) task (as long as it doesn't have dependencies; you dont need to assess all tasks in depth)
  - execute `backlog task view <task id> --plain` and analyze it
  - transition task to "In Progress" (using cli)
  - senior maang-level implement tests (red-green refactor)
  - senior maang-level implement the task (always make sure to re-read and follow [AGENTS.md](AGENTS.md))
  - run tests and check if they're green (if not go back to step 2)
  - re-read the ACs, DoD and implementation plan to see if there is any missing spot or any gaps
  - run formatter
  - check pipeline if its passing (if not go back to step 2)
  - check docs/ and README.md to see if we need to update docs
  - transition task to done (using cli)
  - use conventional and atomic commits to commit all files (including the task file)


PS:
- you dont need to `cd` into the project folder to run commands
- you can use your judment if we'll need to break task into smaller pieces, just follow [backlog-draft-workflow](docs/backlog-draft-workflow.md) to create drafts
- as a senior maang-level employee, feel free to
  - delegate work to your agents
  - question any aspect of the task