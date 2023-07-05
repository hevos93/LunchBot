FROM almalinux
WORKDIR /app
COPY target/release/lunchbot lunchbot
COPY .env .
CMD [ "./lunchbot" ]