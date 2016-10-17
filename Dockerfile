FROM debian:jessie
RUN apt-get update && apt-get install -y \
	build-essential \
	curl \
	vim

RUN curl -sSf https://static.rust-lang.org/rustup.sh | sh
RUN echo "export PATH=~/.cargo/bin:$PATH" >> ~/.bashrc
COPY . /src
WORKDIR /src
