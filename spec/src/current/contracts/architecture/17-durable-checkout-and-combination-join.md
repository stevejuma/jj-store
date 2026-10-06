# Durable checkout and combination join

The installation journal owns original command acknowledgement. Coordinated
execution saves the exact staged combination binding before publication, retains
sealed native completion as pending, selects and syncs that combination, and
acknowledges the command last. Journal-then-combination lock order avoids
inverse nesting. Neither a saved digest nor a metadata fixture grants execution;
provider verification and the owning command engine remain required.
