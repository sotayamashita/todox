# Workflow Retrospective

Inspect the current branch's git history, issue comments, and PR state against each step of `docs/DEVELOPMENT_WORKFLOW.md`. Produce a compliance report showing PASS/FAIL for each workflow step.

## Prerequisites

Before starting, read this file:
- `docs/DEVELOPMENT_WORKFLOW.md` — the authoritative workflow definition

## Step 0: Determine Context

Identify the current branch and linked issue number.

```bash
git branch --show-current
```

Extract the issue number from:
1. Branch name (e.g., `feature/some-description` — look for `#N` in commit messages)
2. Commit messages on this branch: `git log main..HEAD --oneline`
3. PR body (if a PR exists): `gh pr view --json body,number 2>/dev/null`

If no issue number can be determined, stop and ask the user to provide the issue number.

## Step 1: Check — Pick Issue

Verify the issue exists and was assigned.

```bash
gh issue view <N> --json number,title,assignees,state
```

- **PASS** if the issue exists and has at least one assignee
- **FAIL** if the issue has no assignees

## Step 2: Check — Branch Naming

Verify the branch follows the naming convention.

```bash
git branch --show-current
```

Valid prefixes: `feature/`, `fix/`, `refactor/`, `docs/`, `chore/`, `perf/`

- **PASS** if the branch name starts with one of the valid prefixes
- **FAIL** if the branch name does not match any convention

## Step 3: Check — Implementation Plan

Verify that an implementation plan comment was posted on the issue before the first code commit.

```bash
# Get issue comments with timestamps
gh issue view <N> --json comments --jq '.comments[] | {createdAt, body}'

# Get the timestamp of the first commit on this branch
git log main..HEAD --reverse --format='%aI' | head -1
```

- **PASS** if a comment containing "Implementation Plan" or "## Plan" exists and its timestamp is before the first code commit
- **FAIL** if no plan comment exists, or if the plan was posted after the first code commit

## Step 4: Check — TDD

Verify that test files were modified in the branch's commits.

```bash
# List all files changed on this branch
git diff main --name-only
```

Look for files matching:
- `tests/` directory
- `*_test.rs`, `*_test.py`, `*.test.ts`, `*.test.js`, or similar test file patterns

Additionally, check if test changes appear in early commits (TDD means tests come first):

```bash
# Check the first commit for test file changes
git log main..HEAD --reverse --format='%H' | head -1 | xargs git diff-tree --no-commit-id --name-only -r
```

- **PASS** if test files were modified and appear in early commits
- **WARN** if test files were modified but only in later commits (suggests tests written after implementation)
- **FAIL** if no test files were modified (for branches that change `src/` files)
- **SKIP** if the branch only changes docs, config, or non-src files

## Step 5: Check — README Update

If `src/` files were changed, check if `README.md` was also modified (for `feature/` branches only).

```bash
git diff main --name-only
```

- **PASS** if `README.md` was modified, or if no `src/` files were changed
- **WARN** if `src/` files changed on a `feature/` branch but `README.md` was not updated
- **SKIP** if the branch is not a `feature/` branch

## Step 6: Check — Commit Messages

Verify commit messages follow conventional commit format with issue references.

```bash
git log main..HEAD --oneline
```

Expected format: `type(scope): description (#N)` or `type(scope): description`

Valid types: `feat`, `fix`, `test`, `docs`, `refactor`, `chore`, `perf`, `ci`, `style`, `build`

- **PASS** if all commits follow conventional format
- **WARN** if commits follow conventional format but lack issue references
- **FAIL** if any commit does not follow conventional format

## Step 7: Check — Pull Request

Verify a PR exists with proper linking and test plan.

```bash
gh pr view --json number,title,body,state 2>/dev/null
```

Check that:
1. A PR exists for this branch
2. The PR body contains `Closes #<N>` or `Fixes #<N>`
3. The PR body contains a test plan section (`## Test Plan` or `## Test plan`)

- **PASS** if all three conditions are met
- **WARN** if the PR exists but is missing the issue link or test plan
- **FAIL** if no PR exists
- **SKIP** if work is still in progress (no commits yet)

## Output Format

Produce a compliance report in this format:

```
## Workflow Compliance Report

Branch: `<branch-name>`
Issue: #<N> — <issue-title>
Date: <current-date>

| Step | Status | Finding |
|------|--------|---------|
| 1. Pick Issue | PASS/FAIL | Details |
| 2. Branch | PASS/FAIL | Details |
| 3. Plan | PASS/FAIL | Details |
| 4. TDD | PASS/FAIL/WARN/SKIP | Details |
| 5. README | PASS/WARN/SKIP | Details |
| 6. Commits | PASS/WARN/FAIL | Details |
| 7. PR | PASS/WARN/FAIL/SKIP | Details |

### Summary

X/7 steps passed, Y warnings, Z failures

### Violations (if any)

1. **Step N (Name)**: Description of the violation and what should have been done differently.
```

## Post Option

If `$ARGUMENTS` contains `--post`, post the compliance report as a comment on the linked issue:

```bash
gh issue comment <N> --body "<report>"
```
