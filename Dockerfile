FROM rust

WORKDIR /usr/src/janus/
COPY . .

RUN cargo install --path .

EXPOSE 3030

CMD ["janus"]
