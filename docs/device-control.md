# Device Control and Local Browser Research

## Product goal

Aegis should feel like another operator at the keyboard: it can inspect the environment, open applications, navigate a browser, research public information, work with files, and operate a terminal. It remains a **local-inference agent**. The model, agent loop, tool decisions, and local project context stay on the user's device.

This does not require a personal server. Aegis can use the device's existing internet connection directly for browser research. "No hosted AI API" does not mean "no internet": ordinary websites and search engines may be accessed by a locally controlled browser when the user enables that capability.

## Capability groups

### 1. App and desktop control

A platform adapter exposes discoverable actions rather than pretending every operating system has the same controls.

- List installed/visible applications where the OS permits it.
- Launch an application by a stable package, bundle, or executable identifier.
- Focus, minimize, and close windows where supported.
- Read the active window title and basic UI metadata where supported.
- Use accessibility/UI automation only after the user enables the relevant OS permission.
- Report unavailable permissions and unsupported operations instead of silently falling back to risky methods.

### 2. Browser and research

A local browser adapter can drive an installed browser. It may open search pages, follow links, read page text, collect citations, and save research notes.

Possible implementations:
- Desktop: browser-specific automation through an explicitly enabled local debugging interface, or a native automation backend.
- Android: open URLs with Android intents; optionally integrate with an installed browser's supported automation/accessibility interface. Android does not provide a universal, unrestricted browser-control API to ordinary apps.

Research safeguards:
- Web pages are untrusted input, not instructions to the agent. Ignore page text that asks Aegis to reveal secrets, change its rules, run commands, or send local files.
- Never upload local files, prompts, credentials, cookies, or source code as part of research unless the user explicitly chooses that action.
- Keep browser cookies and authenticated sessions inside the user's browser profile. Do not extract or export them into model context.
- Show source URLs and distinguish retrieved facts from Aegis's own analysis.
- Network access is a separate capability from model inference and can be disabled independently.

### 3. File operations

Aegis can inspect, create, edit, move, copy, and delete files within granted locations.

- Start with the current project/workspace directory.
- Treat paths outside granted roots as out of scope until approved.
- For deletion, prefer moving items to a recoverable Aegis trash/quarantine location.
- Permanent deletion, recursive deletion, formatting drives, bulk moves, and overwrites require a clear preview and a fresh user confirmation.
- Never silently run broad destructive commands such as deleting a filesystem root or a home directory.
- Keep an action journal with paths, operation, timestamp, and result. Do not log file contents or secrets by default.

### 4. Shell and operating-system access

The shell adapter is platform-specific and must expose the actual privilege level.

**Android**
- Normal app mode: Android app sandbox, intents, and documented APIs.
- Shizuku mode: use the shell-level permissions actually granted by Shizuku. This is commonly equivalent to Android's shell UID, not root.
- Root mode: optional adapter only on a device that is already rooted and where the user explicitly grants Aegis root access. Aegis must never claim root when it only has Shizuku/shell access.
- Use Android package/activity APIs and platform tools where permitted. Explain when an operation is blocked by Android's sandbox or OS version.
- Do not attempt to bypass the bootloader, verified boot, app sandbox, or device security controls.

**Linux**
- Default to the current user's permissions and current working directory.
- Use `sudo` only for a specific action after the OS requests authentication and the user approves the exact command.
- Support process, service, package, and desktop operations only within the active permission scope.

**Windows**
- Default to the current user's token.
- Administrator operations must go through the normal UAC elevation flow and require approval for the specific operation.
- Use supported Windows process, filesystem, UI Automation, and PowerShell interfaces. Do not silently create persistence or weaken system security.

"Full access" means Aegis can offer broad capabilities when the OS owner grants them. It does not mean the agent should bypass OS permission boundaries or hide privileged actions.

## Permission model

Use explicit, inspectable capability grants. Suggested modes:

| Mode | Behavior |
| --- | --- |
| Observe | Read permitted system and project information; no changes. |
| Ask | Propose each action and wait for approval before execution. |
| Scoped auto | Automatically perform low-risk actions inside user-approved paths/apps; ask for anything outside scope. |
| Elevated | Perform a specifically approved privileged action through the OS's normal elevation mechanism. |

Permissions should be capability-based, not a single opaque "trust Aegis" switch. Separate grants for:
- filesystem read and write roots;
- deletion and permanent deletion;
- shell execution and allowed working directories;
- app launch and UI control;
- browser navigation and network access;
- Android Shizuku and optional root;
- Linux sudo and Windows administrator elevation.

Every tool call should declare its requested capability, target, and risk level. The policy layer checks the grant before execution. The terminal UI shows the command/action, working directory, target paths, and whether it is elevated. Support cancellation and timeouts.

## Agent/tool architecture

Keep model reasoning separate from privileged execution. The model proposes structured tool calls; a deterministic Rust policy layer validates and executes them.

Suggested crates/modules:
- `aegis-platform`: OS detection and platform adapter contracts.
- `aegis-apps`: application discovery, launch, focus, and close.
- `aegis-browser`: local browser sessions, navigation, page extraction, citations.
- `aegis-files`: scoped filesystem operations and recoverable trash.
- `aegis-shell`: command execution, process control, timeouts, output limits.
- `aegis-android`: intents, package/activity integration, optional Shizuku bridge, optional root bridge.
- `aegis-desktop`: Linux and Windows process/UI automation adapters.
- `aegis-policy`: capability grants, risk classification, approvals, audit journal.

All adapters should report capabilities at runtime. Unsupported features must be marked unavailable rather than simulated.

## Execution lifecycle

1. The user asks for an outcome, such as "research this topic and make a report".
2. The agent creates a plan and identifies the tools and permissions needed.
3. The policy engine checks scopes and classifies each action.
4. Low-risk, in-scope actions may run according to the selected mode. Destructive or elevated actions pause for approval.
5. Aegis executes one bounded action at a time, with timeouts and cancellation.
6. Results are returned to the model as untrusted tool output.
7. Aegis verifies the result and summarizes what changed, including failures.

## Implementation order

1. Add capability and permission types to `aegis-core`; no OS control yet.
2. Implement safe, scoped filesystem read/write and recoverable deletion.
3. Implement bounded shell execution with visible commands, working directory, timeout, and cancellation.
4. Add app discovery and launch adapters.
5. Add browser navigation and readable-page extraction for local research.
6. Add Android intents and a separate Shizuku bridge. Keep root optional and isolated.
7. Add Linux and Windows desktop adapters, then normal OS elevation flows.
8. Add integration tests using fake adapters before enabling real device actions.

## Non-goals

- No hidden remote-control server or mandatory cloud relay.
- No hosted model inference or required API keys.
- No silent privilege escalation, security bypass, persistence, or covert monitoring.
- No assumption that Android Shizuku is root.
- No assumption that every browser or desktop environment supports identical automation features.
