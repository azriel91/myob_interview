# Deploying

<!-- https://medium.com/@benmorel/creating-a-linux-service-with-systemd-611b5c8b91d6 -->

These instructions set up `pett_server` to run on an Ubuntu server.

1. Set up SSH configuration to authenticate with the `pett_server`.

    The following assumes access to the server as the `ubuntu` user with passwordless `sudo`:

    **Note:** Make sure to replace the IP address with that of your server.

    ```bash
    # `~/.ssh/config`
    Host pett_server_host
      HostName 127.0.0.1 # Replace this with the server's IP
      User ubuntu
      IdentityFile ~/.ssh/your_private_key
    ```

2. Copy across the session server binary and accompanying files.

    ```bash
    scp ./pett_server         pett_server_host:/home/ubuntu
    scp ./pett_server.service pett_server_host:/home/ubuntu
    ```

3. Log into the server, and set up the `pett_server` service.

    ```bash
    # Log in
    ssh pett_server_host
    ```

    Then:

    ```bash
    sudo mv /home/ubuntu/pett_server.service /etc/systemd/system/
    sudo systemctl start pett_server
    sudo systemctl enable pett_server
    ```
