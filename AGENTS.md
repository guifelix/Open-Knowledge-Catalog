<CRITICAL_INSTRUCTION>

<!-- BACKLOG.MD GUIDELINES START -->
<!-- backlog.md-instructions-version: 1.48.0 -->
## Backlog.md Workflow

This project uses Backlog.md for task and project management.

**For every user request in this project, run `backlog instructions overview` before answering or taking action.**

Use the overview to decide whether to search, read, create, or update Backlog tasks.

Before task lifecycle actions, read the matching detailed guide:
- `backlog instructions task-creation` before creating or splitting tasks
- `backlog instructions task-execution` before planning, changing status or assignee, adding a plan or implementation notes, or implementing task work
- `backlog instructions task-finalization` before checking acceptance criteria, writing final summaries, or moving tasks to terminal statuses

Use `backlog <command> --help` before running unfamiliar commands. Help shows options, fields, and examples.

Do not edit Backlog task, draft, document, decision, or milestone markdown files directly. Use the `backlog` CLI so metadata, relationships, and history stay consistent.
<!-- BACKLOG.MD GUIDELINES END -->

- always use trunk-based development
- always use conventional commits
- always use atomic commits
- Use red-green-refactor when practical
- prefer small functions, clear boundaries and explicit domain language
- prefer resource-shaped APIs over custom actions when the resource model fits
- keep components and modules small enough to review in one pass when practical
- prefer a focused change over a broad refactor unless the task requires the larger move
- do not split the work so far that the behavior becomes hard to verify
- Quality gate before completion: correctness, verification, scope discipline, reliability, maintainability, handoff clarity
- always update task status to "In Progress" when starting a task and "Done" after quality gates are met
- always check README.md (and the referenced docs/) to see if current implementation is currently documented
- always run `cargo test`, `cargo fmt --check`, and `cargo clippy -- -D warnings` before completing any task
- always act as a maang-level employee

</CRITICAL_INSTRUCTION>