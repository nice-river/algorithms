from genericpath import exists
import sys
import argparse
import os


def codechef(action, problem):
    folder = os.path.join(os.path.abspath(__file__), os.pardir, "codechef")
    folder = os.path.abspath(folder)

    if action in ["a", "add"]:
        if not os.path.exists(os.path.join(folder, "src", "bin", f"{problem}.rs")):
            with open(os.path.join(folder, "template.rs")) as temp:
                with open(os.path.join(folder, "src", "bin", f"{problem}.rs"), "w") as target:
                    target.write(temp.read())
        with open(os.path.join(folder, "Cargo.toml"), "r") as cargo:
            lines = cargo.readlines()
        with open(os.path.join(folder, "Cargo.toml"), "w") as cargo:
            for i in range(len(lines)):
                if lines[i].startswith("default-run"):
                    lines[i] = f"default-run = \"{problem}\"\n"
            cargo.writelines(lines)
    elif action in ["r", "rm", "remove"]:
        try:
            os.remove(os.path.join(folder, "src", "bin", f"{problem}.rs"))
        except FileNotFoundError:
            pass
    else:
        raise Exception(f"unknown action {action}")


def codeforces(action, problem):
    folder = os.path.join(os.path.abspath(__file__), os.pardir, "codeforces")
    folder = os.path.abspath(folder)

    if action in ["a", "add"]:
        if not os.path.exists(os.path.join(folder, "src", "bin", f"{problem}.rs")):
            with open(os.path.join(folder, "template.rs")) as temp:
                with open(os.path.join(folder, "src", "bin", f"{problem}.rs"), "w") as target:
                    target.write(temp.read())
        with open(os.path.join(folder, "Cargo.toml"), "r") as cargo:
            lines = cargo.readlines()
        with open(os.path.join(folder, "Cargo.toml"), "w") as cargo:
            for i in range(len(lines)):
                if lines[i].startswith("default-run"):
                    lines[i] = f"default-run = \"{problem}\"\n"
            cargo.writelines(lines)
    elif action in ["r", "rm", "remove"]:
        try:
            os.remove(os.path.join(folder, "src", "bin", f"{problem}.rs"))
        except FileNotFoundError:
            pass
    else:
        raise Exception(f"unknown action {action}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="help me generate oj code templates")
    parser.add_argument("--site", help="which platform am I using")
    parser.add_argument("action", help="add or remove")
    parser.add_argument("problem", help="problem name")

    args = parser.parse_args(sys.argv[1:])
    site = args.site
    globals()[site](args.action, args.problem)
