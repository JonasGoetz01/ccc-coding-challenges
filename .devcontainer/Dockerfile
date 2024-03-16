FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    python3-dev \
    build-essential \
    libssl-dev \
    libffi-dev \
    python3-setuptools \
    python3-venv \
    python3-wheel \ 
    curl \
    git \
    wget \
    zsh \
    nano \
    cmake \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

RUN python3 -m pip install --upgrade pip

#change the default shell to zsh
RUN chsh -s /usr/bin/zsh

RUN curl https://sh.rustup.rs -sSf | zsh -s -- -y

RUN curl -sS https://starship.rs/install.sh | sh -s -- -b /usr/local/bin -y

RUN echo 'eval "$(starship init zsh)"' >> /root/.zshrc

COPY ./starship.toml /root/.config/starship.toml

SHELL ["zsh"]

CMD ["zsh"]
