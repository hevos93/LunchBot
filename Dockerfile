FROM almalinux
WORKDIR /app
COPY target/debug/ .
CMD [ "/app/lunchbot" ]