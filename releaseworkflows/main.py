import git
import re
import time
import os


def print_cloning_animation():
    animation_frames = ["-", "\\", "|", "/"]
    for _ in range(10):
        for frame in animation_frames:
            print("\r\033[37m")
            print(
                f"Cloning repository... {frame}",
                end="",
                flush=True,
            )
            print("\033[0m")
            time.sleep(0.1)


def print_waiting_message():
    print("\033[34mWaiting for tag creation...\033[0m")


def clone_and_initialize_repository(repo_url, local_path):
    if not os.path.exists(local_path):
        print_cloning_animation()
        git.Repo.clone_from(repo_url, local_path)
        print("\033[32mRepository cloned successfully!\033[0m")

    repo = git.Repo(local_path)

    origin = repo.remotes.origin
    origin.fetch()


def get_existing_tags(repo_path):
    repo = git.Repo(repo_path)
    return set(tag.name for tag in repo.tags)


def find_new_tags(repo_path, existing_tags):
    repo = git.Repo(repo_path)
    remote = repo.remotes.origin
    remote.fetch()

    new_tags = set(tag.name for tag in repo.tags) - existing_tags
    return new_tags


def is_semantic_version(tag):
    # flexible regex for semantic versioning with multiple digits
    return re.match(r"^v\d+(\.\d+)+$", tag)


def main():
    github_repo_url = "https://github.com/Arteiii/ReleaseWorkflows.git"
    local_repo_path = "H:/Test/Workflows/ReleaseWorkflows"

    clone_and_initialize_repository(github_repo_url, local_repo_path)

    existing_tags = get_existing_tags(local_repo_path)

    print_waiting_message()

    while True:
        new_tags = find_new_tags(local_repo_path, existing_tags)

        for tag in new_tags:
            if not is_semantic_version(tag):
                print(f"\n\033[33mSyntax not matching for tag: {tag}\033[0m")
            else:
                print(
                    f"\n\033[32mNew semantic version tag found: {tag}\033[0m"
                )

        existing_tags.update(new_tags)

        time.sleep(10)


if __name__ == "__main__":
    main()
