---
applyTo: '**'
---

Unless otherwise instructed:
- KISS - less code, simpler code please! as long as it achieves the end goal
- keep coding style and patterns consistent with the rest of the codebase
- make reasonable assumptions

Unless absolutely necessary, give reasonable assmptions:
- Avoid code comments
- Avoid exception handling
- Avoid condition checks (nil, true, etc.)
- Avoid typeof / isArray etc., prefer to just assign the type when known
- Avoid very long single lines. Maximum of 120 characters per line

Before starting the task:
- Explore files symbols and do web searches as needed to gain context. YOU MUST HAVE 100% confidence in your answer.
- /go/pkg/mod contains all imported libraries source code
- wails/ is our local fork of the wails source code
- should be used for temporary files (build checking, etc.) and also contains experimental projects like wails3-test (which is the wails3 boilerplate template)

During the task
- use pnpm
- MOVE AND THINK FAST

After a task:
- Keep your commentary to a MINIMUM. At most a few short bullet points on the change