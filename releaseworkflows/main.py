import datetime
import git
import re
import time
import os
import logging


import build


def setup_logging():
    logging.basicConfig(
        level=logging.INFO,
        format="%(asctime)s - %(levelname)s - %(message)s",
        filename="build_log.txt",
        filemode="a",
    )


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

    # Get the timestamp for 30 days ago
    thirty_days_ago = datetime.datetime.now() - datetime.timedelta(days=30)

    # Filter tags that are created within the last 30 days
    existing_tags = {
        tag.name for tag in repo.tags if is_recent_tag(tag, thirty_days_ago)
    }

    return existing_tags


def find_new_tags(repo_path, existing_tags):
    repo = git.Repo(repo_path)
    remote = repo.remotes.origin
    remote.fetch()

    # timestamp 30 days ago
    thirty_days_ago = datetime.datetime.now() - datetime.timedelta(days=30)

    for tag in repo.tags:
        if tag.name not in existing_tags and is_recent_tag(
            tag, thirty_days_ago
        ):
            process_tag(tag.name, repo_path)


def is_recent_tag(tag, threshold_date):
    # check if the tag was created after the threshold date
    commit = tag.commit
    commit_date = datetime.datetime.fromtimestamp(commit.committed_date)
    return commit_date >= threshold_date


def is_semantic_version(tag):
    # flexible regex for semantic versioning with multiple digits
    return re.match(r"^v\d+(\.\d+)+$", tag)


def process_tag(tag, local_repo_path):
    try:
        build_instance = build.Build(local_repo_path)

        if not is_semantic_version(tag):
            print(f"Syntax not matching for tag: {tag}")
        else:
            print(f"\033[32mSemantic version tag found: {tag}\033[0m")

            build_instance.compile_code(tag)

    except Exception as e:
        logging.error(f"Error processing tag {tag}: {str(e)}")


def main(
    github_repo_url="https://github.com/Arteiii/ReleaseWorkflows.git",
    local_repo_path="H:/Test/Workflows/ReleaseWorkflows",
):
    setup_logging()

    try:
        clone_and_initialize_repository(github_repo_url, local_repo_path)

        existing_tags = get_existing_tags(local_repo_path)

        for tag in existing_tags:
            process_tag(tag, local_repo_path)

        print("\033[34mWaiting for tag creation...\033[0m")

        while True:
            find_new_tags(local_repo_path, existing_tags)

            time.sleep(10)

    except Exception as e:
        logging.error(f"An unexpected error occurred: {str(e)}")


if __name__ == "__main__":
    main()
