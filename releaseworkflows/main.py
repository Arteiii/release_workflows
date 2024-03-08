import datetime
import git
import re
import time
import os
import logging
import dotenv
import build
import asyncio
from queue import Queue


dotenv.load_dotenv()

# Create a global queue for tasks related to git operations
git_task_queue = Queue()


async def git_remote_fetch(repo_path) -> git.Repo:
    async with git_task_queue.mutex:
        print("Fetch")
        repo = git.Repo(repo_path)
        remote = repo.remotes.origin
        remote.fetch()

        return repo


def setup_logging():
    logging.basicConfig(
        level=logging.INFO,
        format="%(asctime)s - %(levelname)s - %(message)s",
        filename="build_log.txt",
        filemode="a",
    )


async def clone_and_initialize_repository(repo_url, local_path):
    try:
        if not os.path.exists(local_path):
            print("\033[34mCloning repository...\033[0m")
            await git.Repo.clone_from(repo_url, local_path)
        else:
            print("\033[32mRepository already exists.\033[0m")
            return  # skip if repo exists

        repo = git.Repo(local_path)

        origin = repo.remotes.origin
        await origin.fetch()

    except git.exc.GitCommandError as git_error:
        logging.error(f"GitCommandError: {git_error}")
        print("\033[31mError: Repository cloning failed.\033[0m")
        raise

    except Exception as e:
        logging.error(f"An unexpected error occurred: {str(e)}")
        raise

    else:
        print("\033[32mRepository cloned successfully!\033[0m")


async def get_existing_tags(repo_path):
    repo = await git_remote_fetch(repo_path)

    # Get the timestamp for 30 days ago
    thirty_days_ago = datetime.datetime.now() - datetime.timedelta(days=30)

    existing_tags = {
        tag.name
        for tag in repo.tags
        if await is_recent_tag(tag, thirty_days_ago)
    }

    # Process existing tags without awaiting individual tag processing
    for tag in existing_tags:
        print(f"Processing existing tag: {tag}")
        await process_tag(tag, repo_path)

    return existing_tags


async def find_new_tags(repo_path, existing_tags):
    print("\033[34mWaiting for tag creation...\033[0m")

    while True:
        repo = await git_remote_fetch()

        # timestamp 30 days ago
        thirty_days_ago = datetime.datetime.now() - datetime.timedelta(days=30)

        for tag in repo.tags:
            if tag.name not in existing_tags and is_recent_tag(
                tag, thirty_days_ago
            ):
                process_tag(tag.name, repo_path)

        asyncio.sleep(10)


async def is_recent_tag(tag, threshold_date):
    # check if the tag was created after the threshold date
    commit = tag.commit
    commit_date = datetime.datetime.fromtimestamp(commit.committed_date)
    return commit_date >= threshold_date


def is_semantic_version(tag):
    # flexible regex for semantic versioning with multiple digits
    return re.match(r"^v\d+(\.\d+)+$", tag)


async def process_tag(tag, local_repo_path):
    print(f"Processing TAG: {tag}")
    try:
        build_instance = build.Build(local_repo_path)

        if not await is_semantic_version(tag):
            print(f"Syntax not matching for tag: {tag}")
        else:
            print(f"\033[32mSemantic version tag found: {tag}\033[0m")

            await build_instance.compile_code(tag)

    except Exception as e:
        logging.error(f"Error processing tag {tag}: {str(e)}")


async def main(
    github_repo_url: str = "https://github.com/Arteiii/ReleaseWorkflows.git",
    folder_name: str = "ReleaseWorkflows",
    local_base_path: str = os.environ.get("LOCAL_BASE_PATH"),
    github_username: str = os.environ.get("GITHUB_USERNAME"),
    github_access_token: str = os.environ.get("GITHUB_ACCESS_TOKEN"),
    github_ssh_key_path: str = os.environ.get("GITHUB_SSH_KEY_PATH"),
):
    setup_logging()

    try:

        # ANSI escape codes for colors
        yellow_color = "\033[33m"
        white_color = "\033[0m"

        # Print the values for confirmation
        print(f"{yellow_color}GitHub Repo URL: {white_color}{github_repo_url}")
        print(f"{yellow_color}Local Base Path: {white_color}{local_base_path}")

        print(f"{yellow_color}GitHub Username: {white_color}{github_username}")
        print(
            f"{yellow_color}GitHub Access Token: {white_color}{github_access_token}"
        )
        print(
            f"{yellow_color}GitHub SSH KEY: {white_color}{github_ssh_key_path}"
        )

        repo_path = local_base_path + folder_name
        print(
            f"{yellow_color} Local Repo Path: {white_color}{repo_path} \n\n\n"
        )

        await clone_and_initialize_repository(github_repo_url, repo_path)

        existing_tags = await get_existing_tags(repo_path)

        await find_new_tags(repo_path, existing_tags)

    except Exception as e:
        logging.error(f"An unexpected error occurred: {str(e)}")


if __name__ == "__main__":
    asyncio.run(main())
