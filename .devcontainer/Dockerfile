FROM ghcr.io/josiahbull/dotfiles:latest
ARG USERNAME=user-name-goes-here
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN groupadd --gid $USER_GID $USERNAME || true \
  && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
  && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
  && chmod 0440 /etc/sudoers.d/$USERNAME \
  && chsh -s /bin/zsh $USERNAME

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup toolchain install nightly

# TODO: move to base image.
RUN cargo install --locked cargo-insta
RUN cargo install --locked cargo-release
RUN cargo install --locked cargo-semver-checks
RUN cargo install --locked cargo-deny

# Install python tools.
RUN pip3 install pre-commit thefuck

# Copy everything over from /root to /home/$USERNAME
RUN cp -r /root/. /home/$USERNAME
RUN chown -R $USERNAME:$USERNAME /home/$USERNAME

USER $USERNAME
