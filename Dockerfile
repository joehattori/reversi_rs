FROM ubuntu:18.04

RUN mkdir /reversi && apt-get update && \
    apt-get install -y cargo

WORKDIR /reversi

ADD . .
