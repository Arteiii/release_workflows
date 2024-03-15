# Installation Instructions


## Docker

Follow the docker setup instructions [here](https://docs.docker.com/engine/install/).

Verify that Docker CE is installed correctly:

```bash
sudo docker --version
```

This should display the installed version of Docker.

Optionally, you can run Docker as a non-root user:

By default, Docker commands require root privileges.  
To run Docker commands as a non-root user, you can add your user to the docker group:

```bash
sudo usermod -aG docker $USER
```
After adding your user to the docker group, you need to log out and log back in for the changes to take effect.

Start and enable Docker service:

```bash
sudo systemctl start docker
sudo systemctl enable docker
```
