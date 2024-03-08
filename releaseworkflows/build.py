import git
import os
import shutil
from gitignore_parser import parse_gitignore
import json
import hashlib
from typing import List
from pydantic import BaseModel


class TagInfo(BaseModel):
    creator: str
    hash: str
    creation_time: str


class BuildFileInfo(BaseModel):
    file_path: str
    file_hash: str


class RepositoryInfo(BaseModel):
    git_main_branch: str
    git_url: str
    local_path: str
    tags: List[TagInfo]
    build_files: List[BuildFileInfo]


class Build:
    def __init__(self, project_path):
        self.project_path = project_path

    def clone_at_tag(self, tag):
        temp_clone_path = os.path.join(self.project_path, f"temp_clone_{tag}")

        git.Repo.clone_from(
            self.project_path, temp_clone_path, branch=tag, single_branch=True
        )

        return temp_clone_path

    def print_file_tree(self, path):
        try:
            # get gitignore
            gitignore_path = os.path.join(self.project_path, ".gitignore")
            gitignore_patterns = []

            if os.path.exists(gitignore_path):
                with open(gitignore_path, "r") as gitignore_file:
                    gitignore_patterns = gitignore_file.read().splitlines()

            # parse patterns
            gitignore_parser = parse_gitignore(gitignore_patterns)

            print(f"File tree for {path}:")
            for root, dirs, files in os.walk(path):
                # check if current root should be ignored
                if gitignore_parser(root[len(path) + 1 :]):
                    continue

                level = root.replace(path, "").count(os.sep)
                indent = " " * 4 * (level)
                print("{}{}/".format(indent, os.path.basename(root)))
                subindent = " " * 4 * (level + 1)
                for file in files:
                    # check if the current file should be ignored
                    if gitignore_parser(
                        os.path.join(root, file)[len(path) + 1 :]
                    ):
                        continue

                    print("{}{}".format(subindent, file))

        except (OSError, IOError) as e:
            print(f"Error accessing file tree: {str(e)}")
        except Exception as e:
            print(f"An unexpected error occurred: {str(e)}")

    def compile_code(self, tag):
        tag_clone_path = self.clone_at_tag(tag)
        self.print_file_tree(tag_clone_path)

        print(f"Compiling code for tag: {tag}...")

        # TODO: add scripts for compiling and support for docker
        # NOTE: push back in new branch as a only compile option?
        #       creating docker containers how to store settings?
        # .yml scripts like github, only support .sh script that run on the container and sepcify the image to run, other options?

        shutil.rmtree(tag_clone_path)

    def deploy(self, tag):
        tag_clone_path = self.clone_at_tag(tag)
        self.print_file_tree(tag_clone_path)

        print(f"Deploying for tag: {tag}...")

        # TODO: add logi to offer file downloads

        shutil.rmtree(tag_clone_path)
