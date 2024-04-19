FROM ubuntu:latest

RUN apt update && apt install -y \
    build-essential \
    libssl-dev \
    libffi-dev \
    curl \
    git \
    wget \
    nano

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

RUN curl -sS https://starship.rs/install.sh | sh -s -- -b /usr/local/bin -y

RUN echo 'eval "$(starship init bash)"' >> /root/.bashrc

COPY ./starship.toml /root/.config/starship.toml

CMD ["bash"]
