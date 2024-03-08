from fastapi import FastAPI, HTTPException
from pydantic import BaseModel

app = FastAPI()


class RepositoryInfo(BaseModel):
    local_path: str
    git_url: str


@app.get("/")
def read_root():
    return {"Hello": "World"}


@app.post("/initialize-repo")
def initialize_repository(repo_info: RepositoryInfo):
    try:
        # Perform initialization logic here (e.g., clone repository)
        # You can use the values from repo_info.local_path and repo_info.git_url

        # For demonstration purposes, printing the information
        print(
            f"Initializing repository at {repo_info.local_path} with Git URL: {repo_info.git_url}"
        )

        # Return success message or other relevant information
        return {"message": "Repository initialized successfully"}

    except Exception as e:
        # Handle errors and return appropriate HTTP response
        raise HTTPException(
            status_code=500,
            detail=f"Failed to initialize repository: {str(e)}",
        )


# Development startup script
if __name__ == "__main__":
    import uvicorn

    # Run the FastAPI application with auto-reload for development
    uvicorn.run(app, host="127.0.0.1", port=8000, reload=True)
