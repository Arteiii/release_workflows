# ReleaseWorkflows

## FastAPI Server

yee i fucked up git lib no async support (i think)

### Installation

Install dependencies using Poetry:

```bash
poetry install
- NOTE: Poetry installation instructions can be found [here](https://python-poetry.org/docs/).

### Start FastAPI Server

```bash
Copy code
uvicorn main:app
```

## Git Workflows

### Local Tag Creation and Push

Create a new tag and push it to the remote repository:

```bash
git tag v1.0.0
git push origin v1.0.0
```

### Delete Tag

To delete a tag locally and on the remote repository:

```bash
git tag -d v1.0.0
git push origin --delete v1.0.0
```

## Additional Notes

- FastAPI Development:
  During development, run the FastAPI server with auto-reload using main.py

- Dependencies:
  Ensure you have Poetry installed for managing Python dependencies
  