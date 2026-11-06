# Multi-Agent Troubleshooting & Diagnostics

## Common Issues & Diagnoses

### 1. Agent Not Detected
- **Symptom**: `aek agent detect` reports `[INFO] Not currently detected on host`.
- **Cause**: The agent's executable is not on system `PATH` or standard configuration directory does not exist yet.
- **Resolution**: AEK functions in full standalone mode even without agents installed. You can still initialize or export configurations:
  ```bash
  aek agent config export cursor > .cursorrules
  aek agent config export claude-code > CLAUDE.md
  ```

### 2. External Modification Conflicts
- **Symptom**: `aek agent sync` reports collisions or preserves files without updates.
- **Resolution**: Use `--conflict replace-managed` to create automatic backups in `.aek/backups/` while updating files, or inspect differences manually.

### 3. Claude Code Hooks
- **Symptom**: Hooks are not installed into Claude settings automatically.
- **Cause**: By security design, AEK does not install executable shell hooks automatically to prevent unauthorized command execution.
- **Resolution**: Review guidance in `aek agent capabilities claude-code` and add vetted commands to `.claude/settings.json` manually.

### 4. Running the Diagnostics Doctor
```bash
aek agent doctor
```
Provides comprehensive health checks across all four agents.

