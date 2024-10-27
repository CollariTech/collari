FROM rust:latest

WORKDIR /usr/src/app

COPY . .

RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev
RUN git config url.ssh://git@github.com/.insteadOf https://github.com/

RUN mkdir -p -m 0700 ~/.ssh && ssh-keyscan github.com >> ~/.ssh/known_hosts
RUN --mount=type=ssh cargo build --release

CMD ["cargo", "run", "--bin", "rest-server"]