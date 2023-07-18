FROM ubuntu
WORKDIR /app
COPY target/release/lunchbot lunchbot
ENV RUST_LOG info
ENV LUNCHBOT_PORT 4000
RUN apt-get update -y && apt-get upgrade -y && apt-get install wget -y
RUN wget http://nz2.archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.1f-1ubuntu2.19_amd64.deb
RUN dpkg -i libssl1*
CMD [ "./lunchbot" ]