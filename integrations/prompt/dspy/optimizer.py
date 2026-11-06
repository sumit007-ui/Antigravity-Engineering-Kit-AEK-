"""
AEK DSPy teleprompter utilities and signature declarations.
"""

def create_teleprompter(max_bootstrapped_demos=4):
    try:
        import dspy
        from dspy.teleprompt import BootstrapFewShot

        def validation_metric(example, pred, trace=None):
            return len(pred.instruction) > 20

        return BootstrapFewShot(metric=validation_metric, max_bootstrapped_demos=max_bootstrapped_demos)
    except ImportError:
        return None

