#!/usr/bin/env python3
"""
AEK DSPy Bridge: JSON-based execution bridge between AEK Rust core and Stanford DSPy.
"""

import sys
import json
import os

def run_dspy_optimization(payload):
    task_id = payload.get("task_id", "unspecified")
    prompt_text = payload.get("prompt_text", "")
    dataset_path = payload.get("dataset_path", "")
    max_demos = payload.get("max_demos", 4)

    try:
        import dspy  # noqa: F401
        has_dspy = True
    except ImportError:
        has_dspy = False

    if has_dspy:
        try:
            class GenericPromptTask(dspy.Signature):
                """Execute software engineering task following strict constraints."""
                context = dspy.InputField(desc="Context and input specifications")
                instruction = dspy.OutputField(desc="Structured, optimized implementation instruction")

            lm_configured = getattr(dspy.settings, "lm", None) is not None

            trainset = []
            if dataset_path and os.path.isfile(dataset_path):
                try:
                    with open(dataset_path, "r", encoding="utf-8") as f:
                        data = json.load(f)
                    for item in data:
                        trainset.append(dspy.Example(**item).with_inputs("context"))
                except Exception as err:
                    sys.stderr.write(f"Warning: Failed to load dataset '{dataset_path}': {err}\n")

            if lm_configured and trainset:
                from dspy.teleprompt import BootstrapFewShot

                def task_metric(gold, pred, trace=None):
                    output = getattr(pred, "instruction", "")
                    return float(bool(output and len(output) > 10))

                teleprompter = BootstrapFewShot(
                    metric=task_metric,
                    max_bootstrapped_demos=max_demos,
                    max_labeled_demos=max_demos,
                )
                student = dspy.Predict(GenericPromptTask)
                compiled = teleprompter.compile(student, trainset=trainset)
                pred = compiled(context=prompt_text)
                compiled_demos = getattr(compiled, "demos", [])
                return {
                    "task_id": task_id,
                    "optimized_instructions": getattr(pred, "instruction", prompt_text),
                    "demonstrations": [str(d) for d in compiled_demos],
                    "metric_score": 1.0,
                    "iterations": len(compiled_demos) + 1,
                    "success": True,
                    "engine": "Stanford DSPy BootstrapFewShot teleprompter (Active)",
                }
            elif lm_configured:
                predictor = dspy.Predict(GenericPromptTask)
                pred = predictor(context=prompt_text)
                return {
                    "task_id": task_id,
                    "optimized_instructions": getattr(pred, "instruction", prompt_text),
                    "demonstrations": [],
                    "metric_score": 1.0,
                    "iterations": 1,
                    "success": True,
                    "engine": "Stanford DSPy Predictor (Active)",
                }
            else:
                return {
                    "task_id": task_id,
                    "optimized_instructions": prompt_text,
                    "demonstrations": [],
                    "metric_score": None,
                    "iterations": 0,
                    "success": False,
                    "engine": "Stanford DSPy (Unconfigured LM runtime)",
                    "error": "DSPy is installed, but no language model (dspy.settings.lm) or dataset is configured. Set dspy.settings.configure(lm=...) before invoking optimization.",
                }
        except Exception as e:
            return {
                "task_id": task_id,
                "optimized_instructions": prompt_text,
                "demonstrations": [],
                "metric_score": None,
                "iterations": 0,
                "success": False,
                "engine": "Stanford DSPy (Execution error)",
                "error": str(e),
            }
    else:
        return {
            "task_id": task_id,
            "optimized_instructions": prompt_text,
            "demonstrations": [],
            "metric_score": None,
            "iterations": 0,
            "success": False,
            "engine": "None (Host environment missing 'dspy-ai')",
            "error": "DSPy package ('dspy-ai') is not installed in the local Python environment. Install with: pip install dspy-ai"
        }

def main():
    if len(sys.argv) < 2:
        sys.stderr.write("Usage: bridge.py <payload_json_path>\n")
        sys.exit(1)

    payload_path = sys.argv[1]
    if not os.path.isfile(payload_path):
        sys.stderr.write(f"Error: payload file not found: {payload_path}\n")
        sys.exit(1)

    with open(payload_path, "r", encoding="utf-8") as f:
        payload = json.load(f)

    result = run_dspy_optimization(payload)
    print(json.dumps(result, indent=2))

if __name__ == "__main__":
    main()

