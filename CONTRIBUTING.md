# Contribution Guidelines

## General Workflow

1. Fork the `Guitheg/techalib` repository
2. Create a new branch from the `main` branch in your fork.
3. Commit your changes to your branch.
4. Once you have completed your fix, feature or documentation:
    - Rebase upstream changes into your branch.
    - Create a pull request to the `main` branch.
    - Include a detailed description of your changes.
    - Ensure to `allow edits by maintainers` before submitting the pull request.
5. The code changes will be reviewed and tested by the maintainers of the repository.
6. Once the pull request has been reviewed and accepted, it will be merged by a maintainer of the repository.

## Development Environment Setup

1. Dependencies

    To set up your development environment, ensure you have the following installed:

    - [Python 3.8+](https://www.python.org/downloads/)
    - [pip](https://pip.pypa.io/en/stable/)
    - [virtualenv](https://virtualenv.pypa.io/en/latest/) (optional but recommended)
    - [git](https://git-scm.com/)
    - [rust](https://www.rust-lang.org/tools/install)

2. Rust and Cargo

    - Ensuire the repo build and the tests pass
        ```
        cargo build
        cargo test
        ```

3. Python Environment

    You need to create a python environment and install the dependencies in it.

    From the commit create the techalib python environment:
    ```bash
    python3 -m venv .venv --prompt techalib
    source .venv/bin/activate
    pip install -r requirements-dev.txt
    ```
    Also you might need `ta-lib` to generate test data or run benchmark if needed.

4. Build techalib-python with maturin

    Build techalib for python and launch the python tests:
    ```
    maturin develop --release
    pytest
    ```

5. Install pre-commit hook

    ```
    pre-commit install
    ```

## Detailed Workflow

1. Fork the repository

    - Use GitHub's interface to fork the repo.
    - Add the Techalib repo as an upstream remote and fetch upstream data:
        ```
        git remote add upstream https://github.com/Guitheg/techalib.git
        git fetch upstream
        ```

2. Create your branch

    Create your local branch following this naming convention: `feat/branch_name`, `fix/branch_name`, `refactor/branch_name` or `doc/branch_name` from the remote upstream:
    ```
    git checkout -b feat/branch_name upstream/main
    ```

3. (Optional) If you implements a new indicator

    You can use some tools to help you implements a new indicator.

    - Uses `tools/add_new_indicator.py` to create all the files for you required when adding a new indicator:
        ```
        python tools/add_new_indicator.py <fct_name>
        ```
        Certain files will be newly created, while others will be modified. Multiple TODO comments will be inserted across these files; simply address them all.

    - Uses `tools/generated_testdata.py` to create test data for an indicator:
        ```
        python tools/generated_testdata.py <INDICATOR_NAME>
        ```
        If you want to generated the data for a new indicator you will need to modify this file by adding a configuration line such as:
        ```
        CONFIG_DICT = {
            "EMA": Configuration(talib, "EMA", ["close"], dict(timeperiod=30), ["out"]),
            # "YOUR_INDICATOR": Configuration(<python_package>, "PYTH_PCKG_FCT_NAME", ["input_name"], dict(param_name=param_value), ["output_name"])
            ...
        }
        ```

4. Commit Guideline

    Prefix the commit message by one of these:
    - (feat)
    - (fix)
    - (refactor)
    - (cleanup)
    - (doc)

    Commit messages should be written in the present tense, e.g., "(feat) add Super indicator". The first line of your commit message should be a summary of what the commit changes, aiming for about 70 characters max. If you want to explain the commit in more depth, provide a more detailed description after a blank line following the first line.

5. Rebase Upstream Changes

    Rebase your branch onto the latest changes from the main branch in the upstream repository:
    ```
    git pull --rebase upstream development
    ```
    Continue the rebase if conflict occurs:
    ```
    git rebase --continue
    ```
    Ensure all tests pass after rebasing.

6. Create a Pull Request

    Create a clear pull request from your fork and branch to the upstream development branch, detailing your changes. Check 'Allow edits by maintainers' for the maintainers to update your branch with development whenever needed.

    If the maintainers requests changes, make more commits to your branch to address these, then follow this process again from rebasing onwards. Once you've addressed the requests, request further review.
