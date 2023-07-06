FROM almalinux
WORKDIR /app
COPY target/release/lunchbot lunchbot
CMD [ "./lunchbot" ]