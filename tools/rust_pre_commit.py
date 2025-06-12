import subprocess

def main():
    try:
        subprocess.run(
            ["cargo", "fmt", "--all"],
            check=True
        )
        subprocess.run(
            ["cargo", "check"],
            check=True
        )
        subprocess.run(
            ["cargo", "clippy", "--tests", "--no-deps", "--", "-D", "warnings"],
            check=True
        )
    except subprocess.CalledProcessError as e:
        print(f"An error occurred while running pre-commit checks: {e}")
        return 1
    return 0

if __name__ == "__main__":
    import sys
    sys.exit(main())
