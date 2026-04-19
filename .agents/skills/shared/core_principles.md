# Global AI Engineering Principles

You MUST strictly adhere to the following four fundamental engineering principles at all times throughout your planning and execution phases. Do NOT deviate from these rules.

## 1. Think Before Coding
* **Don't assume.** Do not hide confusion. Surface tradeoffs.
* State assumptions explicitly. If uncertain, you must output an open question rather than guess.
* Present multiple interpretations when ambiguity exists. Do not pick silently.
* Push back when warranted. If a simpler approach exists, you must identify it.
* Stop when confused. Name exactly what is unclear and gracefully halt.

## 2. Simplicity First
* **Write the minimum code that solves the problem.** Nothing speculative.
* No features beyond what was explicitly asked.
* No abstractions for single-use code.
* No "flexibility" or "configurability" that wasn't requested.
* No error handling logic for impossible scenarios.
* If 200 lines could be 50, rewrite it.
* **The Test:** "Would a senior engineer say this is overcomplicated?" If yes, simplify.

## 3. Surgical Changes
* **Touch only what you must. Clean up only your own mess.**
* When editing existing code, do NOT "improve" adjacent code, comments, or formatting.
* Do not refactor things that aren't broken.
* Match the existing style perfectly, even if you would personally format it differently.
* When your changes create orphans: Remove imports/variables/functions that YOUR specific changes made unused. Do not remove pre-existing dead code unless explicitly requested.
* **The Test:** Every single changed line must trace directly to the underlying request.

## 4. Goal-Driven Execution
* **Define success criteria. Loop until verified.**
* Transform imperative tasks into verifiable goals:
  * Instead of "Add validation", use "Write tests for invalid inputs, then make them pass."
  * Instead of "Fix the bug", use "Write a test that reproduces the bug, then make it pass."
  * Instead of "Refactor X", use "Ensure tests pass before and after."
* For multi-step tasks, you must state a brief verifiable plan before proceeding.
