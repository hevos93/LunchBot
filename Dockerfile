FROM almalinux
WORKDIR /app
COPY target/release/lunchbot lunchbot
ENV RUST_LOG info
ENV LUNCHBOT_PORT 4000
CMD [ "./lunchbot" ]